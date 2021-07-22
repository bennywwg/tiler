pub mod config;
pub mod preview;
pub mod util;
pub mod image;
pub mod dataset_cache;
pub mod http_api;

use crate::image::*;
/*
use std::fs;
use dataset_cache::DatasetCache;
*/
use config::*;
use glam::*;
use util::*;

#[tokio::main]
async fn main() {
    /*
    let body = reqwest::get("https://spkit.org/datasets/remapped/222_144_000.hgt").await.unwrap()
        .bytes().await.unwrap();

    let image = Image::decode(ImageEncoding::srtm(), &body[..]).unwrap();
    let image_mapped = preview::make_preview(&image, 0.0, 400.0).unwrap();
    */

    let b = math::Dabb2::bounds(ivec2(0,0), ivec2(4,5));

    for idx in b.into_iter() {
        println!("{}", idx);
    }

    return;

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
    
    let config = serde_json::from_str::<http_api::PreviewRequest>(&serde_json::to_string(&preview_request).unwrap());
        println!("{:?}", config);

    http_api::run();
}
