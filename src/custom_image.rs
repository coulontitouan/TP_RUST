mod custom_image {
    use crate::bayer_matrix::BayerMatrix;
    use crate::error_matrix::ErrorMatrix;
    use crate::palette::Palette;
    use crate::{color::Color, custom_image::private_trait::PrivateFunc};
    use image::io::Reader as ImageReader;
    use image::{DynamicImage, GenericImage, GenericImageView, Rgb, Rgba};
    use rand::random;

    pub fn luminance(color: &Rgba<u8>) -> f32 {
        let Rgba([r, g, b, _a]) = color;
        0.2126 * (*r as f32) + 0.7152 * (*g as f32) + 0.0722 * (*b as f32)
    }

    pub fn distance_euclidienne(&color1: &Rgba<u8>, &color2: &Rgba<u8>) -> f32 {
        let Rgba([r1, g1, b1, _a1]) = color1;
        let Rgba([r2, g2, b2, _a2]) = color2;
        ((r1 as f32 - r2 as f32).powi(2)
            + (g1 as f32 - g2 as f32).powi(2)
            + (b1 as f32 - b2 as f32).powi(2))
        .sqrt()
    }

    pub fn open_image(path: &str) -> DynamicImage {
        <DynamicImage as DynamicImageExtensions>::open_image(path)
            .expect(format!("Failed to open image {}", path).as_str())
    }

    pub enum Filter {
        Half,
        BlackAndWhite,
        ColorPalette(Palette),
        RandomDithering,
        OrderedDithering(u8),
        ErrorDiffusion(Palette, ErrorMatrix),
    }

    pub trait DynamicImageExtensions {
        fn open_image(path: &str) -> Result<DynamicImage, image::ImageError>;
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

    impl DynamicImageExtensions for DynamicImage {
        fn open_image(path: &str) -> Result<Self, image::ImageError> {
            Ok(ImageReader::open(format!("images/{}", path))?.decode()?)
        }

        fn set_half_pixels_white(&mut self) -> () {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    if (x + y) % 2 == 0 {
                        self.put_pixel(x, y, Rgba([255, 255, 255, 1]));
                    }
                }
            }
        }

        fn set_color_palette(&mut self, palette: crate::Palette) -> () {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    let pixel = self.get_pixel(x, y);
                    let mut min_distance = f32::MAX;
                    let mut closest_color = Rgba([0, 0, 0, 255]);

                    for palette_color in palette.get_colors() {
                        let Rgb([r, g, b]) = palette_color.rgb();
                        let color = Rgba([r, g, b, 255]);
                        let distance = distance_euclidienne(&pixel, &color);

                        if distance < min_distance {
                            min_distance = distance;
                            closest_color = color;
                        }
                    }

                    self.put_pixel(x, y, closest_color);
                }
            }
        }

        fn set_monochrome_color_palette(&mut self, color1: Color, color2: Color) -> () {
            self.set_custom_dithering(color1, color2, 128.0);
        }

        fn set_black_and_white(&mut self) -> () {
            self.set_monochrome_color_palette(Color::White, Color::Black);
        }

        fn set_random_dithering(&mut self) -> () {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    let pixel = self.get_pixel(x, y);
                    self.put_pixel(
                        x,
                        y,
                        if luminance(&pixel) > random::<f32>() * 255.0 {
                            Rgba([255; 4])
                        } else {
                            Rgba([0, 0, 0, 255])
                        },
                    );
                }
            }
        }

        fn set_ordered_dithering(&mut self, order: u8) -> () {
            let bayer_matrix = BayerMatrix::new(order);
            let bayer_matrix = bayer_matrix.matrix;
            let size = 2_usize.pow(order as u32) as u32;

            for y in 0..self.height() {
                for x in 0..self.width() {
                    let pixel = self.get_pixel(x, y);
                    let threshold = bayer_matrix[(y % size) as usize][(x % size) as usize] as f32
                        * 255.0
                        / (size * size) as f32;

                    self.put_pixel(
                        x,
                        y,
                        if luminance(&pixel) > threshold {
                            Rgba([255; 4])
                        } else {
                            Rgba([0, 0, 0, 255])
                        },
                    );
                }
            }
        }

        fn save_image(self, image_name: &str, extension: &str) {
            let folder_path = "images/output";
            let file_path = format!("{}/{}.{}", folder_path, image_name, extension);

            self.save(file_path.clone())
                .expect(&format!("Failed to save image to {}", file_path));
        }

        fn save_image_png(self, image_name: &str) {
            self.save_image(image_name, "png");
        }

        fn apply_error_diffusion(&mut self, palette: &Palette, error_matrix: &ErrorMatrix) {
            let width = self.width();
            let height = self.height();

            for y in 0..height {
                for x in 0..width {
                    let original_pixel = self.get_pixel(x, y);

                    let closest_color = palette.get_closest_color(original_pixel).rgb();

                    let error = [
                        original_pixel[0] as f32 - closest_color[0] as f32,
                        original_pixel[1] as f32 - closest_color[1] as f32,
                        original_pixel[2] as f32 - closest_color[2] as f32,
                    ];

                    let closest_color =
                        Rgba([closest_color[0], closest_color[1], closest_color[2], 255]);
                    self.put_pixel(x, y, closest_color);

                    for y_error in 0..error_matrix.matrix.len() {
                        for x_error in 0..error_matrix.matrix[y_error].len() {
                            let error_value = error_matrix.get_value(y_error, x_error).unwrap();

                            if x as i32 + (x_error as i32 - error_matrix.x_origin as i32) >= 0
                                && x as i32 + (x_error as i32 - error_matrix.x_origin as i32)
                                    < width as i32
                                && y + (y_error as u32) < height
                            {
                                let mut error_pixel = self.get_pixel(
                                    (x as i32 + (x_error as i32 - error_matrix.x_origin as i32))
                                        as u32,
                                    y + y_error as u32,
                                );

                                for i in 0..3 {
                                    let new_value = (error_pixel[i] as f32
                                        + error[i] * error_value as f32)
                                        .clamp(0.0, 255.0);
                                    error_pixel[i] = new_value as u8;
                                }

                                self.put_pixel(
                                    (x as i32 + (x_error as i32 - error_matrix.x_origin as i32))
                                        as u32,
                                    y + y_error as u32,
                                    error_pixel,
                                );
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

    impl private_trait::PrivateFunc for DynamicImage {
        fn set_custom_dithering(&mut self, color1: Color, color2: Color, threshold: f32) -> () {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    let pixel = self.get_pixel(x, y);

                    let Rgb([r, g, b]) = color1.rgb();
                    let color1_rgba = Rgba([r, g, b, 255]);

                    let Rgb([r, g, b]) = color2.rgb();
                    let color2_rgba = Rgba([r, g, b, 255]);

                    self.put_pixel(
                        x,
                        y,
                        if luminance(&pixel) > threshold {
                            color1_rgba
                        } else {
                            color2_rgba
                        },
                    );
                }
            }
        }
    }
}
