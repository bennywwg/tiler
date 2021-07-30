pub mod config;
pub mod preview;
pub mod util;
pub mod image;
pub mod dataset_cache;
pub mod http_api;
pub mod uri_format;

#[tokio::main]
async fn main() {
    let preview_request = http_api::PreviewRequest {
        min_val: 0.0,
        max_val: 400.0,
        x: 222,
        y: 144,
        z: 0,
        tile_uri_format: "https://67.175.122.81/datasets/remapped/222_144_000.hgt".to_string(),
    };

    println!("{}", serde_urlencoded::to_string(&preview_request).unwrap());
    
    http_api::run().await;
}
