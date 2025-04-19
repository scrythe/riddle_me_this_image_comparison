use image::{self, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
fn main() {
    #[cfg(debug_assertions)]
    normal();
    #[cfg(not(debug_assertions))]
    blazingly_fast();
}

fn normal() {
    println!("Nicht BLAZINGLY FAST genug");
    println!("In Release Mode runnen");
}

fn blazingly_fast() {
    let image1 = image::open("og_file.jpg").expect("og_file.jpg not found");
    let image2 = image::open("copy.png").expect("copy.png not found");
    let (width, height) = image1.dimensions();
    let result_image: RgbaImage = compare_images(image1, image2, width, height);
    result_image
        .save("result.png")
        .expect("Failed to save Image");
}

fn compare_images(
    image1: DynamicImage,
    image2: DynamicImage,
    width: u32,
    height: u32,
) -> RgbaImage {
    let mut result_image: RgbaImage = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let pixel1 = image1.get_pixel(x, y).0;
            let pixel2 = image2.get_pixel(x, y).0;
            let new_pixel = compare_pixels(pixel1, pixel2);
            result_image.put_pixel(x, y, Rgba(new_pixel));
        }
    }
    result_image
}

fn compare_pixels(pixel1: [u8; 4], pixel2: [u8; 4]) -> [u8; 4] {
    if pixel1 != pixel2 {
        return [255, 255, 255, 255];
    }
    return [0, 0, 0, 255];
}

// fn compare_pixels(pixel1: [u8; 4], pixel2: [u8; 4]) -> [u8; 4] {
//     let diff_red = pixel1[0].abs_diff(pixel2[0]);
//     let diff_green = pixel1[0].abs_diff(pixel2[0]);
//     let diff_blue = pixel1[0].abs_diff(pixel2[0]);
//     let pixel_diff = [diff_red, diff_green, diff_blue];
//     let max_diff = pixel_diff.iter().max().unwrap();
//     if *max_diff > 15 {
//         return [255, 255, 255, 255];
//     }
//     return [0, 0, 0, 255];
// }
