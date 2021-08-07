use serde::{Serialize, Deserialize};
use crate::image::{Image, ImageCodec, PixelEncoding};
use glam::*;
use core::time::Duration;
use crate::uri_format::*;
use crate::util::math::*;
use warp::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tilespace {
    pub size: IVec2,
    pub offset: IVec2
}

impl Tilespace {
    pub fn get_covered_tiles(&self, pixel_bounds: Dabb2) -> Dabb2 {
        Dabb2::bounds(
            pixel_bounds.begin.floor_on_interval(self.size),
            pixel_bounds.end.floor_on_interval(self.size) + self.size
        ) / self.size
    }
    pub fn tile_pixels(&self, intput_coord: IVec2) -> Dabb2 {
        (Dabb2::cell(intput_coord) * self.size) + self.offset
    }
    pub fn tile_pixels_level(&self, input_coord: IVec3) -> Dabb2 {
        (Dabb2::cell(ivec2(input_coord.x, input_coord.y)) * (self.size * (1 << input_coord.z))) + self.offset
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetProvider {
    pub tile_uri_format: String,
    pub codec: ImageCodec,
    pub tilespace: Tilespace
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetWriter {
    pub tile_uri_format: String,
    pub codec: ImageCodec,
    pub tilespace: Tilespace
}

fn format_tile_string(template: &str, coord: IVec3) -> Result<String, String> {
    crate::uri_fmt!(&template, {
        "x" => coord.x,
        "y" => coord.y,
        "z" => coord.z
    })
}

impl DatasetProvider {
    fn get_resource_uri(&self, coord: IVec3) -> String {
        match format_tile_string(self.tile_uri_format.as_str(), coord) {
            Ok(str) => str,
            Err(_) => panic!("format string is expected to be valid")
        }            
    }
    pub async fn load_resource(&self, coord: IVec3) -> Option<Image> {
        let client
        =reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build().unwrap();

        let uri
        =crate::uri_fmt!(&self.tile_uri_format, {
            "x" => coord.x,
            "y" => coord.y,
            "z" => coord.z
        }).ok()?;
        
        let bytes
        =client
        .get(uri)
        .send().await.ok()?
        .error_for_status().ok()?
        .bytes().await.ok()?;

        Image::decode(self.codec, &bytes[..]).ok()
    }
}