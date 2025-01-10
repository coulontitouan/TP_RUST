use image::{DynamicImage, GenericImage, GenericImageView, Rgba, Rgb};

trait DynamicImageExtensions {
    fn set_half_pixels_white(&mut self) -> ();
    fn set_black_and_white(&mut self) -> ();
    fn set_monochrome_color_palette(&self, color1: Rgb<u8>, color2: Rgb<u8>) -> ();
}

impl DynamicImageExtensions for DynamicImage {
    fn set_half_pixels_white(&mut self) -> () {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if (x + y) % 2 == 0 {
                    self.put_pixel(x, y, image::Rgba([255, 255, 255, 1]));
                }
            }
        }
    }

    fn set_monochrome_color_palette(&self, color1: [u8;3], color2: [u8;3]) -> () {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.get_pixel(x, y);
                let Rgba([r, g, b, a]) = pixel;
                let luminance = 0.2126 * (*r as f32) + 0.7152 * (*g as f32) + 0.0722 * (*b as f32);
                self.put_pixel(x, y, if luminance > 128.0 { Rgba(color1 } else { color2 });
            }
        }
    }

    fn set_black_and_white(&mut self) -> () {
        self.set_monochrome_color_palette(image::Rgb([255; 3]), image::Rgb([0; 3]));
    }
}

fn black_and_white_image(image: &image::DynamicImage) -> image::DynamicImage {
    return change_color_palette(&image.clone(), image::Rgb([255; 3]), image::Rgb([0; 3]));
}

fn change_color_palette(image: &image::DynamicImage, color1:image::Rgb<u8>, color2:image::Rgb<u8>) -> image::DynamicImage {
    let img = image.to_rgb8();
    let mut new_img = img.clone();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let Rgb([r, g, b]) = pixel;
            let luminance = 0.2126 * (*r as f32) + 0.7152 * (*g as f32) + 0.0722 * (*b as f32);
            new_img.put_pixel(x, y, if luminance > 128.0 { color1 } else { color2 });
        }
    }

    image::DynamicImage::ImageRgb8(new_img)
}

fn save_image_png(image: image::DynamicImage, image_name: &str) {
    save_image(&image, &image_name, "png");
}

fn save_image(image: &image::DynamicImage, image_name: &str, extension: &str) {
    let folder_path = "images/output";
    let file_path = format!("{}/{}.{}", folder_path, image_name, extension);

    image.save(file_path.clone()).expect(&format!("Failed to save image to {}", file_path));
}

fn main() {
    let img = image::open("images/example.jpg").expect("Failed to open image");
    println!("Dimensions: {:?}", img.dimensions());
    println!("Color type: {:?}", img.color());

    let rgb_image = img.clone().to_rgb8();
    save_image_png(image::DynamicImage::ImageRgb8(rgb_image), "rgb_image");

    let mut pngalpha = image::open("images/pngalpha.png").expect("Failed to open image"); // Image avec un canal alpha de base
    pngalpha = image::DynamicImage::ImageRgb8(pngalpha.to_rgb8());
    save_image_png(pngalpha.clone(), "pngalpha");

    let Rgba([r, g, b, a]) = img.get_pixel(32, 52);
    println!("Pixel color - Red: {}, Green: {}, Blue: {}, Alpha: {}", r, g, b, a);

    let mut half_white = img.clone();
    half_white.set_half_pixels_white();
    save_image_png(half_white, "half_white");

    let image_iut = image::open("images/iut.jpg").expect("Failed to open image");
    
    let iut_black_and_white = black_and_white_image(&image_iut);
    save_image_png(iut_black_and_white, "iut_black_and_white");

    let iut_red_and_blue = change_color_palette(&image_iut, image::Rgb([255, 0, 0]), image::Rgb([0, 0, 255]));
    save_image_png(iut_red_and_blue, "iut_red_and_blue");
}