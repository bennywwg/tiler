use serde::{Serialize, Deserialize};
use crate::dataset_cache::*;
use crate::image::{Image, ImageBacked, ImageCodec};
use glam::*;
use core::time::Duration;
use crate::util::math::*;
use crate::network_util::*;

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
    pub tilespace: Tilespace,
    //pub manifest: Vec<IVec3>,
    pub cache: DatasetCache
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
    // tilespace will be whatever size code has, with an offset of 0
    pub async fn create(tile_uri_format: &str, codec: ImageCodec/*, manifest_uri: &str */) -> Result<Self, String> {
        // Verify that tile format can produce a valid result
        format_tile_string(tile_uri_format, ivec3(0,0,0))?;

        Ok(DatasetProvider {
            tile_uri_format: tile_uri_format.to_string(),
            codec,
            tilespace: Tilespace {
                offset: ivec2(0,0),
                size: codec.format.size
            },
            //manifest: parse_json_from_uri(manifest_uri).await?,
            cache: DatasetCache::new(codec.format.raw_size(), 16)
        })
    }
    fn get_resource_uri(&self, coord: IVec3) -> String {
        match format_tile_string(self.tile_uri_format.as_str(), coord) {
            Ok(str) => str,
            Err(_) => panic!("format string is expected to be valid")
        }            
    }
    pub async fn load_resource<'a>(&'a mut self, coord: IVec3) -> Option<ImageBacked<'a>> {
        let uri = self.get_resource_uri(coord);
        match self.cache.access_mut(uri.as_str()) {
            DatasetCacheResult::Valid(backing) => {
                ImageBacked::from_view(self.codec.format, backing).ok()
            },
            DatasetCacheResult::Invalid(backing) => {
                let client
                    =reqwest::Client::builder()
                    .timeout(Duration::from_secs(30))
                    .build().unwrap();
                
                let bytes
                    =client
                    .get(uri)
                    .send().await.ok()?
                    .error_for_status().ok()?
                    .bytes().await.ok()?;

                ImageBacked::decode_into(self.codec, &bytes[..], backing).ok()
            }
        }
    }
}