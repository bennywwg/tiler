use serde::{Serialize, Deserialize};
use crate::dataset_cache::*;
use crate::image::{ImageBacked, ImageCodec};
use glam::*;
use core::time::Duration;
use crate::network_util::*;
use crate::dataset::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetProvider {
    pub tile_uri_format: String,
    pub codec: ImageCodec,
    pub tilespace: Tilespace,
    pub manifest: Vec<IVec3>,
    pub cache: DatasetCache
}

impl TileURIProvider for DatasetProvider {
    fn get_resource_uri(&self, coord: IVec3) -> String {
        match format_tile_string(self.tile_uri_format.as_str(), coord) {
            Ok(str) => str,
            Err(_) => panic!("format string is expected to be valid")
        }            
    }
}

impl DatasetProvider {
    // tilespace will be whatever size code has, with an offset of 0
    pub async fn create(tile_uri_format: &str, codec: ImageCodec, manifest_uri: &str) -> Result<Self, String> {
        // Verify that tile format can produce a valid result
        format_tile_string(tile_uri_format, ivec3(0,0,0))?;

        Ok(DatasetProvider {
            tile_uri_format: tile_uri_format.to_string(),
            codec,
            tilespace: Tilespace {
                offset: ivec2(0,0),
                size: codec.format.size
            },
            manifest: parse_json_from_uri(manifest_uri).await?,
            cache: DatasetCache::new(codec.format.raw_size(), 16)
        })
    }
    pub async fn cache_resource(&mut self, coord: IVec3) -> Result<(),String> {
        if !self.manifest.iter().any(|&c| c == coord) {
            return Ok(());
        }
        let uri = self.get_resource_uri(coord);
        if let DatasetCacheResult::Invalid(backing) = self.cache.access_mut(uri.as_str()) {
            let client
                =reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build().unwrap();
            
            let bytes
                =client
                .get(uri)
                .send().await.map_err(|e| e.to_string())?
                .error_for_status().map_err(|e| e.to_string())?
                .bytes().await.map_err(|e| e.to_string())?;

            ImageBacked::decode_into(self.codec, &bytes[..], backing)?;
        };

        return Ok(());
    }
    pub fn access_cached_resource<'a>(&'a self, coord: IVec3) -> Option<ImageBacked<'a>> {
        ImageBacked::from_view(self.codec.format, self.cache.access(self.get_resource_uri(coord).as_str())?).ok()
    }
}