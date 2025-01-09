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