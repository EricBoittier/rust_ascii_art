extern crate image;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <image_path>", args[0]);
        return;
    }
    let img = image::open(&args[1]).expect("Failed to open image");
    let aspect: f32 = img.height() as f32 / img.width() as f32;
    // println!("w {} h {}", img.width(), img.height());
    // println!("!aspect: {}", aspect);
    let width = 220;
    let img = img.thumbnail(width, (width as f32 * aspect) as u32);
    let img = img.to_luma16();
    let ascii_chars = " .ircoetbOITCEB%";
    let scale = 65535 / (ascii_chars.len() - 1);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let brightness = pixel.0[0];
            let char_index = brightness as usize / scale;
            print!("{}", ascii_chars.chars().nth(char_index).unwrap());
        }
        println!();
    }
}

