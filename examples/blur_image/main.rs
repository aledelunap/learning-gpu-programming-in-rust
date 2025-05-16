mod blur_image;

use image::{self, GrayImage, Luma};

fn load_image(path: &str) -> (Vec<u8>, u32, u32) {
    let gray_img = image::open(path).expect("Failed to open image").to_luma8();

    let (width, height) = gray_img.dimensions();

    let gray_img: Vec<u8> = gray_img.pixels().map(|p| p[0] as u8).collect();

    (gray_img, width, height)
}

fn save_image(path: &str, pixels: Vec<u8>, width: u32, height: u32) {
    let mut img = GrayImage::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let i = (y * width + x) as usize;
        *pixel = Luma([pixels[i]]);
    }

    img.save(path).expect("Failed to save image");
}

fn main() {
    let (image, width, height) = load_image("examples/data/flower.png");

    println!("{width}, {height}");

    let image =
        blur_image::launch::<cubecl::wgpu::WgpuRuntime>(&Default::default(), &image, width, height);

    save_image("examples/data/blurred_flower.png", image, width, height);
}
