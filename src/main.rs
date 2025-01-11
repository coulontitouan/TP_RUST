use image::{open, DynamicImage, GenericImageView, Rgba};
use DynamicImage::ImageRgb8;
include!("palette.rs");
include!("custom_image.rs");

use crate::custom_image::DynamicImageExtensions;

fn save_image_png(image: image::DynamicImage, image_name: &str) {
    save_image(&image, &image_name, "png");
}

fn save_image(image: &image::DynamicImage, image_name: &str, extension: &str) {
    let folder_path = "images/output";
    let file_path = format!("{}/{}.{}", folder_path, image_name, extension);

    image
        .save(file_path.clone())
        .expect(&format!("Failed to save image to {}", file_path));
}

fn main() {
    let img = image::open("images/iut.jpg").expect("Failed to open image");
    println!("Dimensions: {:?}", img.dimensions());
    println!("Color type: {:?}", img.color());

    let rgb_image = img.clone().to_rgb8();
    save_image_png(ImageRgb8(rgb_image), "rgb_image");

    let mut pnglogo = open("images/logo.png").expect("Failed to open image");
    pnglogo = ImageRgb8(pnglogo.to_rgb8());
    save_image_png(pnglogo.clone(), "rgb_logo");

    let mut pngalpha = open("images/pngalpha.png").expect("Failed to open image");
    pngalpha = ImageRgb8(pngalpha.to_rgb8());
    save_image_png(pngalpha.clone(), "rgb_pngalpha");

    let Rgba([r, g, b, a]) = img.get_pixel(32, 52);
    println!(
        "Pixel color - Red: {}, Green: {}, Blue: {}, Alpha: {}",
        r, g, b, a
    );

    let mut half_white = img.clone();
    half_white.set_half_pixels_white();
    save_image_png(half_white, "half_white");

    let image_iut = open("images/iut.jpg").expect("Failed to open image");

    let mut iut_black_and_white = image_iut.clone();
    iut_black_and_white.set_black_and_white();
    save_image_png(iut_black_and_white, "iut_black_and_white");

    let mut iut_red_and_blue = image_iut.clone();
    iut_red_and_blue.set_monochrome_color_palette([255, 0, 0], [0, 0, 255]);
    save_image_png(iut_red_and_blue, "iut_red_and_blue");

    let palette = Palette::new(vec![
        Rgb([0, 0, 0]),       // Noir
        Rgb([255, 0, 0]),     // Rouge
        Rgb([0, 255, 0]),     // Vert
        Rgb([0, 0, 255]),     // Bleu
        Rgb([255, 255, 0]),   // Jaune
        Rgb([255, 0, 255]),   // Magenta
        Rgb([0, 255, 255]),   // Cyan
        Rgb([255, 255, 255]), // Blanc
    ]);

    let mut iut_palette = image_iut.clone();
    iut_palette.set_color_palette(palette);
    save_image_png(iut_palette, "iut_palette");

    let mut iut_dithering = image_iut.clone();
    iut_dithering.set_random_dithering();
    save_image_png(iut_dithering, "iut_dithering");
}
