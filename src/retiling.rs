use crate::image::ImageCodec;
use crate::image::ImageFormat;
use crate::image::{ImageOwned, ImageWriteable};
use crate::image::PixelEncoding;
use crate::util::math::*;
use crate::config::*;
use crate::dataset_writer::*;

use glam::*;
use image::ImageEncoder;
use serde::{Serialize, Deserialize};
use std::primitive;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleRegion {
    pub input_coord: IVec3,                   
    pub pixel_region: Dabb2
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub output_coord: IVec3,
    pub sample_regions: Vec<SampleRegion>
}

pub struct OutputAccumulator {
    pub size: IVec2,
    pub data: Vec<i64>,
    pub samples: Vec<i64>
}

impl OutputAccumulator {
    pub fn index_of(&self, px: &IVec2) -> usize {
        px.y as usize * self.size.x as usize + px.x as usize
    }
    pub fn add_sample<T: num::cast::AsPrimitive<i64>>(&mut self, px: &IVec2, sample: &T) {
        let index = self.index_of(px);
        self.data[index] += sample.as_();
        self.samples[index] += 1;
    }
    fn resolve_templated<T: num::NumCast + num::Integer>(&self, encoding: PixelEncoding) -> ImageOwned {
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
            (8,  true)  => { self.resolve_templated::<i8 >(encoding) },
            (16, true)  => { self.resolve_templated::<i16>(encoding) },
            (32, true)  => { self.resolve_templated::<i32>(encoding) },
            (64, true)  => { self.resolve_templated::<i64>(encoding) },
            (8,  false) => { self.resolve_templated::<u8 >(encoding) },
            (16, false) => { self.resolve_templated::<u16>(encoding) },
            (32, false) => { self.resolve_templated::<u32>(encoding) },
            (64, false) => { self.resolve_templated::<u64>(encoding) },
            _ => panic!()
        }
    }
}

async fn add_samples(dp: &mut DatasetProvider, dw: &DatasetWriter, job: Job) {
    let output_pixel_begin = dw.tilespace.tile_pixels_level(job.output_coord).begin;

    for region in job.sample_regions.into_iter() {
        let image = match dp.load_resource(region.input_coord).await {
            Some(img) => img,
            None => continue
        };

        let input_coord_pixel_begin = dp.tilespace.tile_pixels_level(region.input_coord).begin;

        for pixel in region.pixel_region.into_iter() {
            //let my_pixel = (pixel + input_coord_pixel_begin - output_pixel_begin) >> region.input_coord.z;

            //Samples.AddSample(my_pixel, Data[Pixel.y * Conf.InputTileSize.x + Pixel.x]);
        }
        
        println!("pretending to output {:?}", region);
    }
}

pub fn gen_jobs(dp: &DatasetProvider, dw: &DatasetWriter, pixel_region: Dabb2, begin_level: i32, end_level: i32) -> Vec<Job> {
    (end_level..=begin_level).flat_map(|level| {
        dw.tilespace
        .get_covered_tiles(pixel_region)
        .into_iter()
        .map(|out_coord_2| { 
            let output_coord = ivec3(out_coord_2.x, out_coord_2.y, level);
            let out_pixel_region = dw.tilespace.tile_pixels_level(output_coord);
            let sample_regions = 
                dp.tilespace
                .get_covered_tiles(out_pixel_region)
                .into_iter().map(|input_coord| {
                    let input_texel_region = dp.tilespace.tile_pixels(input_coord);

                    SampleRegion {
                        input_coord: ivec3(input_coord.x, input_coord.y, 0),
                        pixel_region: (out_pixel_region & input_texel_region) - input_texel_region.begin
                    }
                }).collect();
            Job {
                output_coord,
                sample_regions
            }
        }).collect::<Vec<Job>>()
    }).collect()
}