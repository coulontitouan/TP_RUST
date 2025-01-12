use image::{open, DynamicImage, GenericImageView};
use DynamicImage::ImageRgb8;
include!("color.rs");
include!("palette.rs");
include!("custom_image.rs");
include!("bayer_matrix.rs");
include!("error_matrix.rs");
include!("argh.rs");

use crate::bayer_matrix::BayerMatrix;
use crate::color::Color;
use crate::custom_argh::Args;
use crate::custom_image::{luminance, open_image, DynamicImageExtensions};
use crate::error_matrix::{ErrorMatrix, ErrorMatrixType};
use crate::palette::Palette;

fn build() {
    let image_iut = open_image("iut.jpg");

    let rgb_image = image_iut.to_rgb8();
    rgb_image
        .save("images/output/iut.png")
        .expect("Failed to save image");

    let logo_image = open_image("logo.png");
    let rgb_logo = logo_image.to_rgb8();
    rgb_logo
        .save("images/output/rgb_logo.png")
        .expect("Failed to save image");

    let pngalpha_image = open_image("pngalpha.png");
    let rgb_pngalpha = pngalpha_image.to_rgb8();
    rgb_pngalpha
        .save("images/output/rgb_pngalpha.png")
        .expect("Failed to save image");

    let pixel = image_iut.get_pixel(32, 52);
    println!("Pixel (32, 52) : {:?}", pixel);

    let mut half_white = rgb_image.clone();

    for (x, y, pixel) in half_white.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Color::White.rgb();
        }
    }

    half_white
        .save("images/output/half_white.png")
        .expect("Failed to save image");

    println!("Luminance du pixel (32, 52) : {}", luminance(&pixel));

    let mut iut_black_and_white = image_iut.clone();
    iut_black_and_white.set_black_and_white();
    iut_black_and_white.save_image_png("iut_black_and_white");

    let mut iut_red_and_blue = image_iut.clone();
    let custom_blue = Color::new(0, 0, 255);
    iut_red_and_blue.set_monochrome_color_palette(Color::Red, custom_blue);
    iut_red_and_blue.save_image_png("iut_red_and_blue");

    let palette = Palette::new(vec![
        Color::Black,
        Color::White,
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ]);

    let mut iut_palette = image_iut.clone();
    iut_palette.set_color_palette(palette);
    iut_palette.save_image_png("iut_palette");

    let mut iut_dithering = image_iut.clone();
    iut_dithering.set_random_dithering();
    iut_dithering.save_image_png("iut_dithering");

    let bayer_matrix = BayerMatrix::new(3);
    println!("{:?}", bayer_matrix.matrix);
    println!("{:?}", bayer_matrix.is_valid()); // Vérifie si la matrice est valide les nombres doivent être compris entre 0 et 2^order - 1 et etre tous présents

    let mut iut_ordered_dithering = image_iut.clone();
    iut_ordered_dithering.set_ordered_dithering(4);
    iut_ordered_dithering.save_image_png("iut_ordered_dithering");

    let black_and_white_palette = Palette::new(vec![Color::Black, Color::White]);

    let mut iut_error_diffusion = image_iut.clone();
    iut_error_diffusion.apply_error_diffusion(
        &black_and_white_palette,
        &ErrorMatrix::get_type(ErrorMatrixType::Basic),
    );
    iut_error_diffusion.save_image_png("iut_error_diffusion");

    let mut bwrgb_palette = black_and_white_palette.clone();
    bwrgb_palette.add_colors(vec![Color::Red, Color::Green, Color::Blue]);

    let mut iut_2vois_5coul = image_iut.clone();
    iut_2vois_5coul.apply_error_diffusion(
        &bwrgb_palette,
        &ErrorMatrix::get_type(ErrorMatrixType::Basic),
    );
    iut_2vois_5coul.save_image_png("iut_2vois_5coul");

    let david = open_image("david.png");
    let mut david_error_floyd_steinberg = david.clone();
    david_error_floyd_steinberg.apply_error_diffusion(
        &black_and_white_palette,
        &ErrorMatrix::get_type(ErrorMatrixType::JarvisJudiceNinke),
    );
    david_error_floyd_steinberg.save_image_png("david_error_floyd_steinberg");
}

fn main() {
    let args: Args = argh::from_env();

    if args.build {
        build();
        return;
    }

    println!("Input: images/{}", args.get_input());

    let mut image = open(format!("images/{}", args.get_input())).expect("Failed to open image");

    if !image.color().has_alpha() {
        image = ImageRgb8(image.to_rgb8());
    }

    if args.get_filter() {
        let filter = args.get_filter_type();
        image.apply_filter(filter);
    }

    println!("Output: images/output/{}", args.get_output());

    image.save_image_png(&format!("{}", args.get_output()));
}
