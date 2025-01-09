use image::{GenericImageView, Rgba};

fn main() {
    let img = image::open("images/example.jpg").expect("Failed to open image");
    println!("Dimensions: {:?}", img.dimensions());
    println!("Color type: {:?}", img.color());
    let rgb_image = img.to_rgb8();
    rgb_image.save("images/output/rgb_image.png").expect("Failed to save image");

    let logo = image::open("images/logo.png").expect("Failed to open image");
    let rgb_logo = logo.to_rgb8();
    rgb_logo.save("images/output/rgb_logo.png").expect("Failed to save image");

    get_pixel_color(&img, 32, 52);

    let halfed_image = half_pixels_white(&img);
    halfed_image.save("images/output/halfed_image.png").expect("Failed to save image");
}

fn get_pixel_color(image: &image::DynamicImage, x: u32, y: u32) -> Rgba<u8> {
    let pixel = image.get_pixel(x, y);
    match pixel {
        Rgba([r, g, b, a]) => {
            println!("Pixel color at ({}, {}): R: {}, G: {}, B: {}, A: {}", x, y, r, g, b, a);
            Rgba([r, g, b, a])
        }
    }
}

fn half_pixels_white(image: &image::DynamicImage) -> image::DynamicImage {
    let mut img = image.to_rgb8();

    for y in 0..img.height() {
        for x in 0..img.width() {
            if (x + y) % 2 == 0 {
                img.put_pixel(x, y, image::Rgb([255, 255, 255]));
            }
        }
    }

    image::DynamicImage::ImageRgb8(img)
}