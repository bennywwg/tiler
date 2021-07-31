use crate::util::math::*;
use crate::config::*;

use glam::*;
use serde::{Serialize, Deserialize};
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

}

async fn add_samples(dp: &DatasetProvider, dw: &DatasetWriter, job: Job) {
    let output_pixel_begin = dw.tilespace.tile_pixels_level(job.output_coord).begin;

    for region in job.sample_regions.into_iter() {
        /*let image = match dp.load_resource(region.input_coord).await {
            Some(img) => img,
            None => continue;
        };*/

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