mod custom_image {
    use crate::bayer_matrix::BayerMatrix;
    use crate::error_matrix::ErrorMatrix;
    use crate::palette::Palette;
    use crate::{color::Color, custom_image::private_trait::PrivateFunc};
    use image::io::Reader as ImageReader;
    use image::{DynamicImage, Rgb, RgbImage};
    use rand::random;

    pub fn luminance(color: &Rgb<u8>) -> f32 {
        let Rgb([r, g, b]) = color;
        0.2126 * (*r as f32) + 0.7152 * (*g as f32) + 0.0722 * (*b as f32)
    }

    pub fn distance_euclidienne(&color1: &Rgb<u8>, &color2: &Rgb<u8>) -> f32 {
        let Rgb([r1, g1, b1]) = color1;
        let Rgb([r2, g2, b2]) = color2;
        ((r1 as f32 - r2 as f32).powi(2)
            + (g1 as f32 - g2 as f32).powi(2)
            + (b1 as f32 - b2 as f32).powi(2))
        .sqrt()
    }

    fn open_image(path: &str) -> Result<DynamicImage, image::ImageError> {
        let img = ImageReader::open(format!("images/{}", path))?
            .decode()?
            .to_rgb8();
        Ok(DynamicImage::ImageRgb8(img))
    }

    pub fn open_handle_error(path: &str) -> DynamicImage {
        match open_image(path) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Erreur avec l'image {} : {}", path, e);
                std::process::exit(1);
            }
        }
    }

    pub enum Filter {
        Half,
        BlackAndWhite,
        ColorPalette(Palette),
        RandomDithering,
        OrderedDithering(u8),
        ErrorDiffusion(Palette, ErrorMatrix),
    }

    pub trait ImageBufferExtensions {
        fn save_image(self, image_name: &str, extension: &str);
        fn save_image_png(self, image_name: &str);
        fn set_half_pixels_white(&mut self) -> ();
        fn set_color_palette(&mut self, palette: crate::Palette) -> ();
        fn set_black_and_white(&mut self) -> ();
        fn set_monochrome_color_palette(&mut self, color1: Color, color2: Color) -> ();
        fn set_random_dithering(&mut self) -> ();
        fn set_ordered_dithering(&mut self, order: u8) -> ();
        fn apply_error_diffusion(&mut self, palette: &Palette, error_matrix: &ErrorMatrix);
        fn apply_filter(&mut self, filter: Filter);
    }

    impl ImageBufferExtensions for RgbImage {
        fn save_image(self, image_name: &str, extension: &str) {
            let folder_path = "images/output";
            let file_path = format!("{}/{}.{}", folder_path, image_name, extension);

            self.save(file_path.clone())
                .expect(&format!("Failed to save image to {}", file_path));
        }

        fn save_image_png(self, image_name: &str) {
            self.save_image(image_name, "png");
        }

        fn set_half_pixels_white(&mut self) -> () {
            for (x, y, pixel) in self.enumerate_pixels_mut() {
                if (x + y) % 2 == 0 {
                    *pixel = Color::White.rgb();
                }
            }
        }

        fn set_color_palette(&mut self, palette: crate::Palette) -> () {
            for (_x, _y, pixel) in self.enumerate_pixels_mut() {
                let mut min_distance = f32::MAX;
                let mut closest_color = Color::Black.rgb();

                for palette_color in palette.get_colors() {
                    let distance = distance_euclidienne(&pixel, &palette_color.rgb());

                    if distance < min_distance {
                        min_distance = distance;
                        closest_color = palette_color.rgb();
                    }
                }

                *pixel = closest_color;
            }
        }

        fn set_monochrome_color_palette(&mut self, color1: Color, color2: Color) -> () {
            self.set_custom_dithering(color1, color2, 128.0);
        }

        fn set_black_and_white(&mut self) -> () {
            self.set_monochrome_color_palette(Color::White, Color::Black);
        }

        fn set_random_dithering(&mut self) -> () {
            for (_x, _y, pixel) in self.enumerate_pixels_mut() {
                *pixel = if luminance(&pixel) > random::<f32>() * 255.0 {
                    Color::White.rgb()
                } else {
                    Color::Black.rgb()
                };
            }
        }

        fn set_ordered_dithering(&mut self, order: u8) -> () {
            let bayer_matrix = BayerMatrix::new(order);
            let bayer_matrix = bayer_matrix.matrix;
            let size = 2_usize.pow(order as u32) as u32;

            for (x, y, pixel) in self.enumerate_pixels_mut() {
                let threshold = bayer_matrix[(y % size) as usize][(x % size) as usize] as f32
                    * 255.0
                    / (size * size) as f32;

                *pixel = if luminance(&pixel) > threshold {
                    Color::White.rgb()
                } else {
                    Color::Black.rgb()
                };
            }
        }

        fn apply_error_diffusion(&mut self, palette: &Palette, error_matrix: &ErrorMatrix) {
            let width = self.width();
            let height = self.height();

            let mut error_buffer = vec![vec![[0.0; 3]; width as usize]; height as usize];

            for (x, y, pixel) in self.enumerate_pixels_mut() {
                let mut original_pixel = [
                    pixel[0] as f32 + error_buffer[y as usize][x as usize][0],
                    pixel[1] as f32 + error_buffer[y as usize][x as usize][1],
                    pixel[2] as f32 + error_buffer[y as usize][x as usize][2],
                ];

                for channel in original_pixel.iter_mut() {
                    *channel = channel.clamp(0.0, 255.0);
                }

                let closest_color = palette
                    .get_closest_color(Rgb([
                        original_pixel[0] as u8,
                        original_pixel[1] as u8,
                        original_pixel[2] as u8,
                    ]))
                    .rgb();

                let error = [
                    original_pixel[0] - closest_color[0] as f32,
                    original_pixel[1] - closest_color[1] as f32,
                    original_pixel[2] - closest_color[2] as f32,
                ];

                *pixel = closest_color;

                for y_error in 0..error_matrix.matrix.len() {
                    for x_error in 0..error_matrix.matrix[y_error].len() {
                        let error_value = error_matrix.get_value(y_error, x_error).unwrap();

                        let target_x = x as i32 + (x_error as i32 - error_matrix.x_origin as i32);
                        let target_y = y as i32 + (y_error as i32);

                        if target_x >= 0
                            && target_x < width as i32
                            && target_y >= 0
                            && target_y < height as i32
                        {
                            let target_x = target_x as usize;
                            let target_y = target_y as usize;

                            for i in 0..3 {
                                error_buffer[target_y][target_x][i] +=
                                    error[i] * error_value as f32;
                            }
                        }
                    }
                }
            }
        }

        fn apply_filter(&mut self, filter: Filter) {
            match filter {
                Filter::Half => self.set_half_pixels_white(),
                Filter::BlackAndWhite => self.set_black_and_white(),
                Filter::ColorPalette(palette) => self.set_color_palette(palette),
                Filter::RandomDithering => self.set_random_dithering(),
                Filter::OrderedDithering(order) => self.set_ordered_dithering(order),
                Filter::ErrorDiffusion(palette, error_matrix) => {
                    self.apply_error_diffusion(&palette, &error_matrix)
                }
            }
        }
    }

    mod private_trait {
        use crate::color::Color;
        pub trait PrivateFunc {
            fn set_custom_dithering(&mut self, color1: Color, color2: Color, threshold: f32) -> ();
        }
    }

    impl private_trait::PrivateFunc for image::ImageBuffer<Rgb<u8>, Vec<u8>> {
        fn set_custom_dithering(&mut self, color1: Color, color2: Color, threshold: f32) -> () {
            for (_x, _y, pixel) in self.enumerate_pixels_mut() {
                *pixel = if luminance(&pixel) > threshold {
                    color1.rgb()
                } else {
                    color2.rgb()
                };
            }
        }
    }
}
