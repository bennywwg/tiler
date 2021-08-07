use crate::{config::Tilespace, image::ImageCodec};
use glam::*;

pub mod config;
pub mod preview;
pub mod util;
pub mod image;
pub mod dataset_cache;
pub mod http_api;
pub mod uri_format;
pub mod retiling;
pub mod serde_json_warp;

#[tokio::main]
async fn main() {
    // let dp = config::DatasetProvider {
    //     tile_uri_format: "https://67.175.122.81/datasets/remapped/222_144_000.hgt".to_string(),
    //     codec: ImageCodec::srtm(),
    //     tilespace: Tilespace {
    //         size: ivec2(1201, 1201),
    //         offset: ivec2(0, 0)
    //     }
    // };

    // let dw = config::DatasetWriter {
    //     tile_uri_format: "https://67.175.122.81/datasets/remapped/222_144_000.hgt".to_string(),
    //     codec: ImageCodec::srtm(),
    //     tilespace: Tilespace {
    //         size: ivec2(512, 512),
    //         offset: ivec2(0, 0)
    //     }
    // };

    // let jobs = retiling::gen_jobs(&dp, &dw, util::math::Dabb2::bounds(ivec2(0,0), ivec2(2000,2000)), 0, 0);
    // println!("{:?}", jobs);

    // return;

    let preview_request = http_api::PreviewRequest {
        tile_uri_format: "https://spkit.org/datasets/srtm/remapped/{x:3}_{y:3}_{z:3}.hgt".to_string(),
        decode_info: Some(ImageCodec::srtm()),
        coord: ivec3(222, 144, 0),
        range: vec2(0.0, 400.0),
    };

    //let s = serde_json::to_string(&preview_request).unwrap();
    //println!("{}", s);
    //println!("{:?}", serde_qs::from_str::<http_api::PreviewRequest>(&s));
    
    http_api::run().await;
}
