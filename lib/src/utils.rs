use anyhow::Result;
use image::{ImageBuffer, Rgb, RgbImage};
use std::io::Cursor;

pub fn image_to_bytes(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut cursor = Cursor::new(&mut data);
    RgbImage::write_to(&image, &mut cursor, image::ImageOutputFormat::Png)?;

    Ok(data)
}
