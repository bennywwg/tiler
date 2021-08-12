use glam::*;
use crate::image::*;

pub struct SampleAccumulator {
    pub size: IVec2,
    pub data: Vec<i64>,
    pub samples: Vec<i64>
}

impl SampleAccumulator {
    pub fn new(size: IVec2) -> Self {
        let total_size = size.x as usize * size.y as usize;
        SampleAccumulator {
            size,
            data: vec![0; total_size],
            samples: vec![0; total_size]
        }
    }
    pub fn index_of(&self, px: &IVec2) -> usize {
        px.y as usize * self.size.x as usize + px.x as usize
    }
    pub fn add_sample<T: num::cast::AsPrimitive<i64>>(&mut self, px: &IVec2, sample: &T) {
        let index = self.index_of(px);
        self.data[index] += sample.as_();
        self.samples[index] += 1;
    }
    pub fn resolve_templated<T: num::NumCast + num::Integer>(&self, encoding: PixelEncoding) -> ImageOwned {
        let mut res = ImageOwned::empty_new(ImageFormat { encoding, size: self.size });
        for y in 0..self.size.y {
            let line_index = y as usize * self.size.x as usize;
            for x in 0..self.size.x {
                let index = line_index + x as usize;
                *res.get_pixel_mem::<T>(index * std::mem::size_of::<T>()) = num::cast(self.data[index] / self.samples[index]).unwrap();
            }
        }
        res
    }
    pub fn resolve(&self, encoding: PixelEncoding) -> ImageOwned {
        match (encoding.bit_depth, encoding.signed) {
            (8 , true ) => self.resolve_templated::<i8 >(encoding),
            (16, true ) => self.resolve_templated::<i16>(encoding),
            (32, true ) => self.resolve_templated::<i32>(encoding),
            (64, true ) => self.resolve_templated::<i64>(encoding),
            (8 , false) => self.resolve_templated::<u8 >(encoding),
            (16, false) => self.resolve_templated::<u16>(encoding),
            (32, false) => self.resolve_templated::<u32>(encoding),
            (64, false) => self.resolve_templated::<u64>(encoding),
            _ => panic!()
        }
    }
    pub fn clear(&mut self) {
        for i in &mut self.data[..] { *i = 0; }
        for i in &mut self.samples[..] { *i = 0; }
    }
}
