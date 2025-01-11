use image::Rgb;

pub struct Palette {
    colors: Vec<Rgb<u8>>,
}

impl Palette {
    pub fn new(colors: Vec<Rgb<u8>>) -> Self {
        Palette { colors }
    }

    pub fn get_colors(&self) -> &Vec<Rgb<u8>> {
        &self.colors
    }

    pub fn add_color(&mut self, color: Rgb<u8>) {
        self.colors.push(color);
    }
}