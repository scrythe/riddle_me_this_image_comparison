use image::{self, GenericImageView, ImageBuffer, Rgba, RgbaImage};
fn main() {
    let og_image = image::open("riddle1.jpg").expect("File not found");
    let image_with_secret = image::open("secret.png").expect("File not found");
    let (width, height) = og_image.dimensions();
    let mut result_image: RgbaImage = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let og_pixel = og_image.get_pixel(x, y).0;
            let secret_pixel = image_with_secret.get_pixel(x, y).0;
            let new_pixel = if secret_pixel != [0, 0, 0, 255] {
                let mut pixel = og_pixel;
                pixel[0] -= 1;
                pixel
            } else {
                og_pixel
            };
            result_image.put_pixel(x, y, Rgba(new_pixel));
        }
    }
    result_image
        .save("riddle2.png")
        .expect("Failed to save image");
}
