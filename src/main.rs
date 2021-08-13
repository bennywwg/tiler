use crate::{image::{ImageCodec, ImageFormat, PixelEncoding}, retiling::process_all_jobs, util::math};
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
        "https://spkit.org/datasets/srtm/remapped/{x:3}_{y:3}_{z:3}.hgt",
        ImageCodec::srtm(),
        "https://spkit.org/datasets/srtm/remapped/manifest.json"
    ).await {
        Ok(dp) => dp,
        Err(_) => { return; }
    };

    let dw = dataset_writer::DatasetWriter {
        tile_uri_format: "./output/{x:3}_{y:3}_{z:3}.png".to_string(),
        codec: ImageCodec {
            filetype: image::ImageFiletype::PNG,
            format: ImageFormat {
                encoding: PixelEncoding::srtm(),
                size: ivec2(512, 512)
            }
        },
        tilespace: dataset::Tilespace {
            size: ivec2(512, 512),
            offset: ivec2(0, 0)
        },
        filetype: image::ImageFiletype::PNG
    };

    println!("Created Dataset Provider, generating jobs...");

    let out_tile = ivec2(3, 225);
    let jobs = retiling::gen_jobs(
        &dp,
        &dw,
        math::Dabb2::cell(out_tile) * 512,
        0,
        0
    );

    println!("{}", serde_json::to_string(&jobs).unwrap());

    process_all_jobs(&mut dp, &dw, &jobs).await;

    //let s = serde_json::to_string(&preview_request).unwrap();
    //println!("{}", s);
    //println!("{:?}", serde_json::from_str::<http_api::PreviewRequest>(&s));
    
    //http_api::run().await;
}
