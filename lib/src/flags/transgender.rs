use crate::flag::FlagModule;
use crate::utils::image_to_bytes;
use anyhow::Result;
use bitvec::prelude::*;

use image::{ImageBuffer, Rgb, RgbImage};
const BLUE: Rgb<u8> = Rgb([91, 206, 250]);
const PINK: Rgb<u8> = Rgb([245, 169, 184]);
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

const QBIT_COLORS: [Rgb<u8>; 16] = [
    Rgb([254, 254, 254]),
    Rgb([253, 253, 253]),
    Rgb([252, 252, 252]),
    Rgb([251, 251, 251]),
    Rgb([250, 250, 250]),
    Rgb([249, 249, 249]),
    Rgb([248, 248, 248]),
    Rgb([247, 247, 247]),
    Rgb([246, 246, 246]),
    Rgb([245, 245, 245]),
    Rgb([244, 244, 244]),
    Rgb([243, 243, 243]),
    Rgb([242, 242, 242]),
    Rgb([241, 241, 241]),
    Rgb([240, 240, 240]),
    Rgb([239, 239, 239]),
];

fn qbit_to_color(qbit: u8) -> Rgb<u8> {
    QBIT_COLORS[qbit as usize]
}

fn color_to_qbit(color: Rgb<u8>) -> Option<u8> {
    QBIT_COLORS
        .iter()
        .position(|&c| c == color)
        .map(|pos| pos as u8)
}

#[derive(Debug, Clone, Copy)]
pub struct Transgender {
    pub width: u32,
    pub height: u32,
}

impl Transgender {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for Transgender {
    fn default() -> Self {
        Self::new(1024, 512)
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
        let unencoded_bits: BitVec<u8, Lsb0> = data.into_iter().collect();
        let mut chunked_bits = unencoded_bits.chunks_exact(4);

        let mut image = RgbImage::new(self.width, self.height);
        let colors = [BLUE, PINK, WHITE, PINK, BLUE];

        if chunked_bits.len() * 4 > (self.width * self.height * 4) as usize {
            return Err(anyhow::anyhow!(
                "The data is too large to encode in this resolution, need at least {} pixels",
                chunked_bits.len()
            ));
        }

        image.enumerate_pixels_mut().for_each(|(_x, y, pixel)| {
            let color_index = (y * 5 / self.height) as usize;
            let color = colors[color_index];

            if color == WHITE {
                let qbit = match chunked_bits.next() {
                    Some(chunk) => chunk.load::<u8>(),
                    None => {
                        return *pixel = WHITE;
                    }
                };

                *pixel = qbit_to_color(qbit);
            } else {
                *pixel = color;
            }
        });

        image_to_bytes(image)
    }

    fn decode_bytes(&self, data: impl Into<Vec<u8>>) -> Result<Vec<u8>> {
        let image = image::load_from_memory(&data.into())?.to_rgb8();
        let mut decoded_bits = BitVec::<u8, Lsb0>::new();

        image.enumerate_pixels().for_each(|(_x, y, pixel)| {
            let color_index = (y * 5 / self.height) as usize;
            let color = match color_index {
                0 => BLUE,
                1 => PINK,
                2 => WHITE,
                3 => PINK,
                4 => BLUE,
                _ => unreachable!(),
            };

            if color == WHITE {
                let qbit = match color_to_qbit(*pixel) {
                    Some(qbit) => qbit,
                    None => return,
                };

                decoded_bits.extend_from_bitslice(&qbit.view_bits::<Lsb0>()[..4]);
            }
        });

        let bytes = decoded_bits.into_vec();
        Ok(bytes)
    }

    fn is_valid(&self, data: impl Into<Vec<u8>>) -> bool {
        let data = data.into();
        let image = image::load_from_memory(&data[..]).unwrap().to_rgb8();

        let mut bits = BitVec::<u8, Lsb0>::new();

        image.enumerate_pixels().for_each(|(_x, y, pixel)| {
            let color_index = (y * 5 / self.height) as usize;
            let color = match color_index {
                0 => BLUE,
                1 => PINK,
                2 => WHITE,
                3 => PINK,
                4 => BLUE,
                _ => unreachable!(),
            };

            if color == WHITE {
                let qbit = match color_to_qbit(*pixel) {
                    Some(qbit) => qbit,
                    None => return,
                };

                bits.extend_from_bitslice(&qbit.view_bits::<Lsb0>()[..4]);
            }
        });

        let bytes = bits.into_vec();
        let unencoded_bits: BitVec<u8, Lsb0> = data.into_iter().collect();

        bytes.len() == unencoded_bits.len() / 4
    }
}

#[cfg(test)]
mod tests {
    use crate::flag::FlagModule;

    use super::Transgender;

    #[test]
    fn generate_flag() {
        let flag = Transgender::default();
        flag.generate().unwrap();
    }

    #[test]
    fn encode_decode() {
        let flag = Transgender::default();
        let data = b"Hello, world!";
        let encoded = flag.encode_bytes(data).unwrap();
        let decoded = flag.decode_bytes(encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
    }

    #[test]
    fn encode_overflow() {
        let flag = Transgender::new(10, 10);
        let data = vec![0; 100]; // 100 bytes should not fit in 10x10 pixels

        match flag.encode_bytes(data.clone()) {
            Ok(_) => {
                assert!(false);
                return;
            }
            Err(e) => {
                println!("{}", e);
                assert!(true);
            }
        };
    }
}
