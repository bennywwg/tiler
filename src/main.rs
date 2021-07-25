pub mod config;
pub mod preview;
pub mod util;
pub mod image;
pub mod dataset_cache;
pub mod http_api;
pub mod uri_format;

use crate::config::DatasetConfig;
use glam::*;
use crate::image::ImageEncoding;

#[tokio::main]
async fn main() {
    let preview_request = http_api::PreviewRequest {
        min_val: 0.0,
        max_val: 400.0,
        coord: ivec3(222, 144, 0),
        config: DatasetConfig {
            tile_uri_format: "https://spkit.org/datasets/remapped/222_144_000.hgt".to_string(),
            encoding: Some(ImageEncoding::srtm())
        }
    };

    println!("{}", serde_json::to_string(&preview_request).unwrap());
    
    http_api::run();
}
