use image::{GenericImageView, ImageReader};
use std::fs::File;
use std::io::Write;

fn main() {
    // Load the BMP image
    let img = ImageReader::open("image.bmp")
        .expect("Failed to open image file")
        .decode()
        .expect("Failed to decode image")
        .to_luma8();

    println!("H {} x {} w", img.height(), img.width());

    // Create a vector to hold the binary data
    let mut binary_data = Vec::new();

    // Iterate over each pixel in the image
    let mut i = 0;
    for y in 0..img.height() {
        for x in 0..img.width() / 8 {
            let mut pixel_val = 0;
            for i in 0..8 {
                let pixel = img.get_pixel(x * 8 + i, y);
                let bit = if pixel.0[0] == 255 { 1u8 } else { 0u8 };
                pixel_val |= bit << (7 - i);
            }
            println!("{i} {x},{y}:{pixel_val}");
            binary_data.push(pixel_val);
            i += 1;
        }
    }

    // Write the binary data to a raw file
    let mut raw_file = File::create("output.raw").expect("Failed to create raw file");
    raw_file.write_all(&binary_data).expect("Failed to write to raw file");

    println!("Conversion complete!");
}

