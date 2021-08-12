use crate::{image::ImageCodec, retiling::process_all_jobs};
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
pub mod network_util;
pub mod dataset;
pub mod dataset_writer;
pub mod sample_accumulator;

#[tokio::main]
async fn main() {
    
    let mut dp = match config::DatasetProvider::create(
        "https://67.175.122.81/datasets/remapped/{x:3}_{y:3}_{z:3}.hgt",
        ImageCodec::srtm(),
        "https://spkit.org/datasets/srtm/remapped/manifest.json"
    ).await {
        Ok(dp) => dp,
        Err(_) => { return; }
    };

    let dw = dataset_writer::DatasetWriter {
        tile_uri_format: "./output/{x:3}_{y:3}_{z:3}.hgt".to_string(),
        codec: ImageCodec::srtm(),
        tilespace: dataset::Tilespace {
            size: ivec2(512, 512),
            offset: ivec2(0, 0)
        },
        filetype: image::ImageFiletype::PNG
    };

    let jobs = retiling::gen_jobs(&dp, &dw, util::math::Dabb2::bounds(ivec2(0,0), ivec2(2000,2000)), 0, 0);

    process_all_jobs(&mut dp, &dw, &jobs).await;

    //let s = serde_json::to_string(&preview_request).unwrap();
    //println!("{}", s);
    //println!("{:?}", serde_json::from_str::<http_api::PreviewRequest>(&s));
    
    //http_api::run().await;
}
