use crate::image::Image;
use crate::util::math::*;
use crate::config::*;
use crate::dataset_writer::*;
use crate::dataset::*;

use glam::*;
use serde::{Serialize, Deserialize};
use std::vec::Vec;
use crate::sample_accumulator::*;

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

async fn add_samples_templated<T>(dp: &mut DatasetProvider, dw: &DatasetWriter, job: &Job, samples: &mut SampleAccumulator)
    where T: num::NumCast + num::cast::AsPrimitive<i64> + num::Integer {
    let output_pixel_begin = dw.tilespace.tile_pixels_level(job.output_coord).begin;

    for region in job.sample_regions.iter() {
        let divisor = 1 >> region.input_coord.z;

        if let Err(_) = dp.cache_resource(region.input_coord).await {
            continue;
        }

        // ugh. Limitation of 
        let image = match dp.access_cached_resource(region.input_coord) {
            Some(image) => image,
            None => {
                continue;
            }
        };

        let input_coord_pixel_begin = dp.tilespace.tile_pixels_level(region.input_coord).begin;

        for pixel in region.pixel_region.into_iter() {
            let val = image.get_pixel_shared::<T>((dp.codec.format.size - pixel - 1) / divisor);
            samples.add_sample(dw.codec.format.size - (pixel + (input_coord_pixel_begin - output_pixel_begin)) - 1, val);
        }
    }

    if samples.num_samples != 0 {
        println!("Generated {} with {} samples", dw.get_resource_uri(job.output_coord), samples.num_samples);
        if let Err(str) = dw.write_tile(job.output_coord, &samples.resolve_templated::<T>(dw.codec.format.encoding)) {
            println!("Unexpected tile write error: {}", str);
        }
    }

    return;
}

pub async fn process_all_jobs_templated<T: num::NumCast + num::cast::AsPrimitive<i64> + num::Integer>(dp: &mut DatasetProvider, dw: &DatasetWriter, jobs: &Vec<Job>) {
    let mut samples = SampleAccumulator::new(dw.codec.format.size);
    for job in jobs.iter() {
        add_samples_templated::<T>(dp, dw, job, &mut samples).await;
        samples.clear();
    }
}

pub async fn process_all_jobs(dp: &mut DatasetProvider, dw: &DatasetWriter, jobs: &Vec<Job>) {
    let encoding = dw.codec.format.encoding;
    match (encoding.bit_depth, encoding.signed) {
        (8 , true ) => process_all_jobs_templated::<i8 >(dp, dw, jobs).await,
        (16, true ) => process_all_jobs_templated::<i16>(dp, dw, jobs).await,
        (32, true ) => process_all_jobs_templated::<i32>(dp, dw, jobs).await,
        (64, true ) => process_all_jobs_templated::<i64>(dp, dw, jobs).await,
        (8 , false) => process_all_jobs_templated::<u8 >(dp, dw, jobs).await,
        (16, false) => process_all_jobs_templated::<u16>(dp, dw, jobs).await,
        (32, false) => process_all_jobs_templated::<u32>(dp, dw, jobs).await,
        (64, false) => process_all_jobs_templated::<u64>(dp, dw, jobs).await,
        _ => panic!()
    }
}

pub fn gen_jobs(dp: &DatasetProvider, dw: &DatasetWriter, pixel_region: Dabb2, begin_level: i32, end_level: i32) -> Vec<Job> {
    (end_level..=begin_level).flat_map(|level| {
        dw.tilespace
        .get_covered_tiles(pixel_region)
        .into_iter()
        .filter_map(|out_coord_2| { 
            let output_coord = ivec3(out_coord_2.x, out_coord_2.y, level);
            let out_pixel_region = dw.tilespace.tile_pixels_level(output_coord);
            let sample_regions: Vec<SampleRegion>
                =dp.tilespace
                .get_covered_tiles(out_pixel_region)
                .into_iter()
                .filter(|input_coord| {
                    dp.manifest.iter().any(|coord| {
                        *coord == ivec3(input_coord.x, input_coord.y, 0)
                    })
                })
                .map(|input_coord| {
                    let input_texel_region = dp.tilespace.tile_pixels(input_coord);

                    SampleRegion {
                        input_coord: ivec3(input_coord.x, input_coord.y, 0),
                        pixel_region: (out_pixel_region & input_texel_region) - input_texel_region.begin
                    }
                })
                .collect();
            match sample_regions.is_empty() {
                true  => None,
                false => Some(Job {
                    output_coord,
                    sample_regions
                })
            }
        }).collect::<Vec<Job>>()
    }).collect()
}