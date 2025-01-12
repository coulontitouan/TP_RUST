mod color {
    use image::Rgb;

    #[derive(Clone, Copy, Debug)]
    pub enum Color {
        Black,
        White,
        Red,
        Green,
        Blue,
        Yellow,
        Magenta,
        Cyan,
        Custom(u8, u8, u8),
    }

    impl Color {
        pub fn rgb(&self) -> Rgb<u8> {
            match self {
                Color::Black => Rgb([0, 0, 0]),
                Color::White => Rgb([255, 255, 255]),
                Color::Red => Rgb([255, 0, 0]),
                Color::Green => Rgb([0, 255, 0]),
                Color::Blue => Rgb([0, 0, 255]),
                Color::Yellow => Rgb([255, 255, 0]),
                Color::Magenta => Rgb([255, 0, 255]),
                Color::Cyan => Rgb([0, 255, 255]),
                Color::Custom(r, g, b) => Rgb([*r, *g, *b]),
            }
        }

        pub fn from_str(color: &str) -> Result<Self, String> {
            match color.to_lowercase().as_str() {
                "black" => Ok(Color::Black),
                "white" => Ok(Color::White),
                "red" => Ok(Color::Red),
                "green" => Ok(Color::Green),
                "blue" => Ok(Color::Blue),
                "yellow" => Ok(Color::Yellow),
                "magenta" => Ok(Color::Magenta),
                "cyan" => Ok(Color::Cyan),
                _ => Err(format!("Couleur invalide : {}", color)),
            }
        }

        pub fn new(r: u8, g: u8, b: u8) -> Self {
            Color::Custom(r, g, b)
        }
    }

    pub fn parse_colors(colors_str: &str) -> Result<Vec<Color>, String> {
        let mut colors = Vec::new();
        for color in colors_str.split(',') {
            match Color::from_str(color.trim()) {
                Ok(c) => colors.push(c),
                Err(_) => return Err(color.to_string()),
            }
        }
        Ok(colors)
    }
}
