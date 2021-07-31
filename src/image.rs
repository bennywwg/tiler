use serde::{Serialize, Deserialize};
use glam::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ImageCompression {
    Raw, PNG, TIFF
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PixelEncoding {
    pub bit_depth: i32,
    pub gamma: f64,
    pub channels: i32,
    pub swap_endian: bool,
    pub signed: bool
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ImageFormat {
    pub encoding: PixelEncoding,
    pub size: IVec2
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ImageCodec {
    pub encoding: PixelEncoding,
    pub compression: ImageCompression,
    pub size: IVec2
}

impl PixelEncoding {
    pub fn srtm() -> Self {
        Self {
            bit_depth: 16,
            gamma: 1.0,
            channels: 1,
            swap_endian: false,
            signed: true
        }
    }

    pub fn color() -> Self {
        Self {
            bit_depth: 8,
            channels: 3,
            gamma: 1.0,
            swap_endian: false,
            signed: false
        }
    }
}

impl ImageCodec {
    pub fn srtm() -> Self {
        Self {
            encoding: PixelEncoding::srtm(),
            compression: ImageCompression::Raw,
            size: ivec2(1201, 1201)
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub format: ImageFormat,
    pub data: Vec<u8>
}

impl Image {
    pub fn decode(decode_info: ImageCodec, data: &[u8]) -> Result<Self, String> {
        match decode_info.compression {
            ImageCompression::Raw => {
                if decode_info.size.x as usize * decode_info.size.y as usize * decode_info.encoding.channels as usize * (decode_info.encoding.bit_depth / 8) as usize == data.len() {
                    Ok(Image {
                        format: ImageFormat {
                            encoding: decode_info.encoding,
                            size: decode_info.size 
                        },
                        data: data.to_vec()
                    })
                } else {
                    Err("Image incorrectly encoded".to_string())
                }
            },
            _ => panic!("not implemented")
        }
    }

    pub fn guess_decode(data: &[u8]) -> Result<(Self, ImageCodec), String> {
        Err("Not yet implemented".to_string())
    }
}