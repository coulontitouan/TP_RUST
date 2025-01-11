use image::Rgb;

pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
}

impl Color {
    pub fn to_rgb(&self) -> Rgb<u8> {
        match self {
            Color::Black => Rgb([0, 0, 0]),
            Color::White => Rgb([255, 255, 255]),
            Color::Red => Rgb([255, 0, 0]),
            Color::Green => Rgb([0, 255, 0]),
            Color::Blue => Rgb([0, 0, 255]),
            Color::Yellow => Rgb([255, 255, 0]),
            Color::Magenta => Rgb([255, 0, 255]),
            Color::Cyan => Rgb([0, 255, 255]),
        }
    }
}