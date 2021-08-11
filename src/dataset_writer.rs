
use serde::{Serialize, Deserialize};
use crate::image::{ImageFiletype, Image, ImageCodec};
use glam::*;
use crate::dataset::*;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetWriter {
    pub tile_uri_format: String,
    pub codec: ImageCodec,
    pub tilespace: Tilespace,
    pub filetype: ImageFiletype
}

impl TileURIProvider for DatasetWriter {
    fn get_resource_uri(&self, coord: IVec3) -> String {
        match format_tile_string(self.tile_uri_format.as_str(), coord) {
            Ok(str) => str,
            Err(_) => panic!("format string is expected to be valid")
        }            
    }
}

impl DatasetWriter {
    pub async fn create(tile_uri_format: &str, codec: ImageCodec, out_filetype: ImageFiletype) -> Result<Self, String> {
        // Verify that tile format can produce a valid result
        format_tile_string(tile_uri_format, ivec3(0,0,0))?;

        Ok(DatasetWriter {
            tile_uri_format: tile_uri_format.to_string(),
            codec,
            tilespace: Tilespace {
                offset: ivec2(0,0),
                size: codec.format.size
            },
            filetype: out_filetype
        })
    }
    pub fn write_tile(&self, coord: IVec3, image: &impl Image) -> Result<(), String> {
        fs::write(
            self.get_resource_uri(coord),
            image.compress(self.filetype)?
        )
        .map_err(|io_er| io_er.to_string())
    }
}