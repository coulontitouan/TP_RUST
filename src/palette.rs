mod palette {
    use image::Rgb;

    use crate::color::Color;
    use crate::custom_image::distance_euclidienne;

    #[derive(Clone)]
    pub struct Palette {
        colors: Vec<Color>,
    }

    impl Palette {
        pub fn new(colors: Vec<Color>) -> Self {
            Palette { colors }
        }

        pub fn get_colors(&self) -> &Vec<Color> {
            &self.colors
        }

        pub fn add_colors(&mut self, colors: Vec<Color>) {
            self.colors.extend(colors);
        }

        pub fn get_closest_color(&self, color: Rgb<u8>) -> Color {
            let mut min_distance = f32::MAX;
            let mut closest_color = Color::Black;

            for palette_color in self.colors.iter() {
                let distance = distance_euclidienne(&color, &palette_color.rgb());

                if distance < min_distance {
                    min_distance = distance;
                    closest_color = *palette_color;
                }
            }

            closest_color
        }
    }
}
