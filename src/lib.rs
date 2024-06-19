use image::{ImageFormat, DynamicImage};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_to_webp(input_image: &[u8], quality: u8) -> Vec<u8> {
    let img = image::load_from_memory(input_image).expect("Failed to load image");
    let mut buffer = Cursor::new(Vec::new());

    // Using the specified format directly in the encoder
    img.write_to(&mut buffer, ImageFormat::WebP).expect("Failed to write WebP");

    buffer.into_inner()
}
