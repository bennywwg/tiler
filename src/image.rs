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

impl ImageFormat {
    pub fn raw_size(&self) -> usize {
        self.size.x as usize * self.size.y as usize * self.encoding.channels as usize * (self.encoding.bit_depth / 8) as usize
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ImageCodec {
    pub format: ImageFormat,
    pub compression: ImageCompression
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
            format: ImageFormat {
                encoding: PixelEncoding::srtm(),
                size: ivec2(1201, 1201)
            },
            compression: ImageCompression::Raw,
        }
    }
}

fn decode_image(codec: ImageCodec, dst: &mut[u8], src: &[u8]) {
    match codec.compression {
        ImageCompression::Raw => {
            if codec.format.raw_size() == src.len() {
                dst.copy_from_slice(src);
            } else {
                panic!("Image incorrectly encoded".to_string());
            }
        },
        _ => panic!("not implemented")
    }
}

pub trait Image {
    fn get_shared_backing(&self) -> &[u8];
    //fn get_backing(&mut self) -> &mut[u8];
    fn get_format(&self) -> ImageFormat;
}

#[derive(Debug)]
pub struct ImageBacked<'a> {
    pub format: ImageFormat,
    pub data: &'a[u8]
}

impl<'a> Image for ImageBacked<'a> {
    fn get_shared_backing(&self) -> &[u8] {
        self.data
    }

    //fn get_backing(&mut self) -> &'a mut[u8] {
    //    self.data
    //}

    fn get_format(&self) -> ImageFormat {
        self.format
    }
}

impl<'a> ImageBacked<'a> {
    pub fn decode_into(decode_info: ImageCodec, data: &[u8], backing: &'a mut[u8]) -> Result<Self, String> {
        decode_image(decode_info, backing, data);
        Ok(Self {
            format: decode_info.format,
            data: backing
        })
    }
    pub fn from_view(format: ImageFormat, backing: &'a [u8]) -> Result<Self, String> {
        Ok(ImageBacked {
            format,
            data: backing
        })
    }
}

pub struct ImageOwned {
    pub format: ImageFormat,
    pub data: Vec<u8>
}

impl ImageOwned {
    pub fn decode_new(decode_info: ImageCodec, data: &[u8]) -> Result<Self, String> {
        let mut vec: Vec<u8> = vec![0; decode_info.format.raw_size()];
        decode_image(decode_info, &mut vec[..], data);
        Ok(ImageOwned {
            format: decode_info.format,
            data: vec
        })
    }
}

impl ImageOwned {
    fn get_shared_backing(&self) -> &[u8] {
        &self.data[..]
    }

    //fn get_backing(&mut self) -> & mut[u8] {
    //    &self.data[..]
    //}
}