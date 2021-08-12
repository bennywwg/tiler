use serde::{Serialize, Deserialize};
use glam::*;
use ::image as image_ext;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ImageFiletype {
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
    pub filetype: ImageFiletype
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
            filetype: ImageFiletype::Raw,
        }
    }
}

fn decode_image(codec: ImageCodec, dst: &mut[u8], src: &[u8]) {
    match codec.filetype {
        ImageFiletype::Raw => {
            if codec.format.raw_size() == src.len() {
                dst.copy_from_slice(src);
            } else {
                panic!("{}", "Image incorrectly encoded".to_string());
            }
        },
        _ => panic!("not implemented")
    }
}

pub trait Image {
    fn get_shared_backing(&self) -> &[u8];
    //fn get_backing(&mut self) -> &mut[u8];
    fn get_format(&self) -> ImageFormat;
    fn compress(&self, filetype: ImageFiletype) -> Result<Vec<u8>,String> {
        return match filetype {
            ImageFiletype::Raw => {
                let mut res = vec![];
                res.extend_from_slice(self.get_shared_backing());
                Ok(res)
            },
            ImageFiletype::PNG => {
                let a =
                image_ext::RgbImage::from_raw(
                    self.get_format().size.x as u32,
                     self.get_format().size.y as u32,
                      self.get_shared_backing().to_vec()).ok_or("Failed to create image")?;
                let d = image_ext::DynamicImage::ImageRgb8(a);
        
                let mut res = vec![];
                d.write_to(&mut res, image_ext::ImageOutputFormat::Png).map_err(|e| e.to_string())?;
        
                Ok(res)
            },
            ImageFiletype::TIFF => panic!()
        }
    }
    fn get_pixel_mem_shared<T: num::Integer>(&self, mem_index: usize) -> &T {
        assert!(mem_index + std::mem::size_of::<T>() <= self.get_shared_backing().len());
        unsafe { std::mem::transmute::<&u8, &T>(&self.get_shared_backing()[mem_index]) }
    }
    fn get_pixel_shared<T: num::Integer>(&self, px: IVec2) -> &T {
        self.get_pixel_mem_shared(px.y as usize * self.get_format().size.x as usize + px.x as usize)
    }
}

pub trait ImageWriteable : Image {
    fn get_backing(&mut self) -> &mut[u8];
    fn get_pixel_mem<T: num::Integer>(&mut self, mem_index: usize) -> &mut T {
        assert!(mem_index + std::mem::size_of::<T>() <= self.get_backing().len());
        unsafe { std::mem::transmute::<&mut u8, &mut T>(&mut self.get_backing()[mem_index]) }
    }

    // Recommended to use get_pixel_mem for repeated operations for performance reasons
    // that function should compile down to nothing
    fn get_pixel<T: num::Integer>(&mut self, px: IVec2) -> &mut T {
        self.get_pixel_mem(px.y as usize * self.get_format().size.x as usize + px.x as usize)
    }
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
    pub fn empty_new(format: ImageFormat) -> Self {
        ImageOwned {
            format: format,
            data: vec![0; format.raw_size()]
        }
    }
}

impl Image for ImageOwned {
    fn get_shared_backing(&self) -> &[u8] {
        &self.data[..]
    }
    fn get_format(&self) -> ImageFormat {
        self.format
    }
}

impl ImageWriteable for ImageOwned {
    fn get_backing(&mut self) -> &mut[u8] {
        &mut self.data[..]
    }
}