mod custom_image {
    use crate::custom_image::private_trait::PrivateFunc;
    use rand::random;
    use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

    pub trait DynamicImageExtensions {
        fn set_half_pixels_white(&mut self) -> ();
        fn set_color_palette(&mut self, palette: crate::Palette) -> ();
        fn set_black_and_white(&mut self) -> ();
        fn set_monochrome_color_palette(&mut self, color1: [u8; 3], color2: [u8; 3]) -> ();
        fn set_random_dithering(&mut self) -> ();
    }

    impl DynamicImageExtensions for DynamicImage {
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
                    let Rgba([r, g, b, a]) = pixel;
                    let mut min_distance = f32::MAX;
                    let mut closest_color = Rgba([0, 0, 0, 255]);

                    for palette_color in palette.get_colors() {
                        let color =
                            Rgba([palette_color[0], palette_color[1], palette_color[2], 255]);
                        let distance = distance_euclidienne(&Rgba([r, g, b, a]), &color);

                        if distance < min_distance {
                            min_distance = distance;
                            closest_color = color;
                        }
                    }

                    self.put_pixel(x, y, closest_color);
                }
            }
        }

        fn set_monochrome_color_palette(&mut self, color1: [u8; 3], color2: [u8; 3]) -> () {
            self.set_custom_dithering(color1, color2, 128.0);
        }

        fn set_black_and_white(&mut self) -> () {
            self.set_monochrome_color_palette([255; 3], [0; 3]);
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
    }

    fn luminance(color: &Rgba<u8>) -> f32 {
        let Rgba([r, g, b, _a]) = color;
        0.2126 * (*r as f32) + 0.7152 * (*g as f32) + 0.0722 * (*b as f32)
    }

    fn distance_euclidienne(&color1: &Rgba<u8>, &color2: &Rgba<u8>) -> f32 {
        let Rgba([r1, g1, b1, _a1]) = color1;
        let Rgba([r2, g2, b2, _a2]) = color2;
        ((r1 as f32 - r2 as f32).powi(2)
            + (g1 as f32 - g2 as f32).powi(2)
            + (b1 as f32 - b2 as f32).powi(2))
        .sqrt()
    }

    mod private_trait {
        pub trait PrivateFunc {
            fn set_custom_dithering(
                &mut self,
                color1: [u8; 3],
                color2: [u8; 3],
                threshold: f32,
            ) -> ();
        }
    }

    impl private_trait::PrivateFunc for DynamicImage {
        fn set_custom_dithering(&mut self, color1: [u8; 3], color2: [u8; 3], threshold: f32) -> () {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    let pixel = self.get_pixel(x, y);
                    self.put_pixel(
                        x,
                        y,
                        if luminance(&pixel) > threshold {
                            Rgba([color1[0], color1[1], color1[2], 255])
                        } else {
                            Rgba([color2[0], color2[1], color2[2], 255])
                        },
                    );
                }
            }
        }
    }
}
