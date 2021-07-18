use serde::{Serialize, Deserialize};
use glam::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageCompression {
    Raw, PNG, TIFF
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEncoding {
    pub size: IVec2,
    pub bit_depth: i32,
    pub gamma: f64,
    pub channels: i32,
    pub swap_endian: bool,
    pub signed: bool,
    pub compression: ImageCompression
}

impl ImageEncoding {
    pub fn srtm() -> Self {
        Self {
            size: ivec2(1201, 1201),
            bit_depth: 16,
            gamma: 1.0,
            channels: 1,
            swap_endian: false,
            signed: true,
            compression: ImageCompression::Raw
        }
    }

    pub fn color(size: IVec2, compression: ImageCompression) -> Self {
        Self {
            compression,
            size: size,
            bit_depth: 8,
            channels: 3,
            gamma: 1.0,
            swap_endian: false,
            signed: false
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub encoding: ImageEncoding,
    pub data: Vec<u8>
}

impl Image {
    pub fn decode(encoding: ImageEncoding, data: &[u8]) -> Result<Self, String> {
        match encoding.compression {
            ImageCompression::Raw => {
                if encoding.size.x as usize * encoding.size.y as usize * encoding.channels as usize * (encoding.bit_depth / 8) as usize == data.len() {
                    Ok(Image { encoding, data: data.to_vec() })
                } else {
                    Err("Image incorrectly encoded".to_string())
                }
            },
            _ => panic!("not implemented")
        }
    }
}