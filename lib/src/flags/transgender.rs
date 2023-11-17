use crate::flag::FlagModule;
use crate::utils::image_to_bytes;
use anyhow::Result;
use bitreader::BitReader;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::{
    fs,
    io::{Cursor, Write},
};

const BLUE: Rgb<u8> = Rgb([91, 206, 250]);
const PINK: Rgb<u8> = Rgb([245, 169, 184]);
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

#[derive(Debug, Clone, Copy)]
pub struct Transgender {
    width: u32,
    height: u32,
}

impl Transgender {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for Transgender {
    fn default() -> Self {
        Self::new(128, 64)
    }
}

impl FlagModule for Transgender {
    fn name(&self) -> &str {
        "Transgender"
    }

    fn generate(&self) -> Result<Vec<u8>> {
        let colors = [BLUE, PINK, WHITE, PINK, BLUE];
        let mut image = ImageBuffer::new(self.width, self.height);
        image.enumerate_pixels_mut().for_each(|(_x, y, pixel)| {
            let color_index = (y * 5 / self.height) as usize;
            *pixel = colors[color_index];
        });

        image_to_bytes(image)
    }

    fn encode_bytes(&self, data: impl Into<Vec<u8>>) -> Result<Vec<u8>> {
        let data = data.into();

        let mut image = RgbImage::new(self.width, self.height);
        let mut reader = BitReader::new(&data);

        let colors = [BLUE, PINK, WHITE, PINK, BLUE];
        image.enumerate_pixels_mut().for_each(|(_x, y, pixel)| {
            let color_index = (y * 5 / self.height) as usize;
            let color = colors[color_index];

            if color == WHITE {
                let color = match reader.read_u8(4) {
                    Ok(0) => Rgb([254, 254, 254]),
                    Ok(1) => Rgb([253, 253, 253]),
                    Ok(2) => Rgb([252, 252, 252]),
                    Ok(3) => Rgb([251, 251, 251]),
                    Ok(4) => Rgb([250, 250, 250]),
                    Ok(5) => Rgb([249, 249, 249]),
                    Ok(6) => Rgb([248, 248, 248]),
                    Ok(7) => Rgb([247, 247, 247]),
                    Ok(8) => Rgb([246, 246, 246]),
                    Ok(9) => Rgb([245, 245, 245]),
                    Ok(10) => Rgb([244, 244, 244]),
                    Ok(11) => Rgb([243, 243, 243]),
                    Ok(12) => Rgb([242, 242, 242]),
                    Ok(13) => Rgb([241, 241, 241]),
                    Ok(14) => Rgb([240, 240, 240]),
                    Ok(15) => Rgb([239, 239, 239]),
                    _ => Rgb([255, 255, 255]),
                };

                *pixel = color;
            } else {
                *pixel = color;
            }
        });

        image_to_bytes(image)
    }

    fn decode_bytes(&self, data: impl Into<Vec<u8>>) -> Result<Vec<u8>> {
        let data = data.into();
        let image = image::load_from_memory(&data[..])?.to_rgb8();
    
        let mut writer = Cursor::new(Vec::new());
        let mut color_index = 0;
        let mut color = BLUE;
    
        for (x, y, pixel) in image.enumerate_pixels() {
            if color == WHITE {
                let qbit = match pixel {
                    Rgb([254, 254, 254]) => 0,
                    Rgb([253, 253, 253]) => 1,
                    Rgb([252, 252, 252]) => 2,
                    Rgb([251, 251, 251]) => 3,
                    Rgb([250, 250, 250]) => 4,
                    Rgb([249, 249, 249]) => 5,
                    Rgb([248, 248, 248]) => 6,
                    Rgb([247, 247, 247]) => 7,
                    Rgb([246, 246, 246]) => 8,
                    Rgb([245, 245, 245]) => 9,
                    Rgb([244, 244, 244]) => 10,
                    Rgb([243, 243, 243]) => 11,
                    Rgb([242, 242, 242]) => 12,
                    Rgb([241, 241, 241]) => 13,
                    Rgb([240, 240, 240]) => 14,
                    Rgb([239, 239, 239]) => 15,
                    _ => continue, // Skip non-white pixels
                };
    
                writer.write(&[qbit]).unwrap();
            }
    
            if y * 5 / self.height != color_index {
                color_index = y * 5 / self.height;
                color = match color_index {
                    0 => BLUE,
                    1 => PINK,
                    2 => WHITE,
                    3 => PINK,
                    4 => BLUE,
                    _ => BLUE,
                };
            }
        }
    
        Ok(writer.into_inner())
    }

    fn is_valid(&self, data: impl Into<Vec<u8>>) -> bool {
        true
    }
}

#[test]
fn test_generate() {
    let flag = Transgender::default();
    let data = flag.generate();

    fs::write("output.png", data.unwrap()).unwrap();
}

#[test]
fn test_encode_decode() {
    let data = "Hello World";
    let flag = Transgender::default();

    let encoded = flag.encode_bytes(data).unwrap();
    fs::write("output.png", &encoded).unwrap();

    let decoded = flag.decode_bytes(encoded).unwrap();
    println!("Original: {:?}", String::from_utf8_lossy(data.as_bytes()));
    println!("Decoded: {:?}", String::from_utf8_lossy(&decoded));
    assert_eq!(decoded, data.as_bytes());
}
