use core::time::Duration;
use serde::{Serialize, Deserialize};
use glam::*;
use warp::*;

use crate::image::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviewRequest {
    pub tile_uri_format: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub min_val: f32,
    pub max_val: f32
}

macro_rules! warp_reject {
    ($name: ident) => {
        #[derive(Debug)]
        struct $name;
        impl warp::reject::Reject for $name {}
    };
    ($type: ty as $name: ident) => {
        #[derive(Debug)]
        struct $name($type);
        impl warp::reject::Reject for $name {}
        impl std::ops::Deref for $name {
            type Target = $type;
            fn deref(&self) -> &<Self as std::ops::Deref>::Target {
                &self.0
            }
        }
    }
}

warp_reject!(PreviewGenerateError);
warp_reject!(String as UriFormatError);
warp_reject!(String as ImageDecodeError);
warp_reject!(reqwest::Error as ReqwestError);

async fn hello_world(r: PreviewRequest) -> Result<impl warp::reply::Reply, warp::Rejection>{
    let encoding = ImageEncoding::srtm();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(999))
        .build().unwrap();

    let uri = crate::uri_fmt!(&r.tile_uri_format, {
        "x" => r.x,
        "y" => r.y,
        "z" => r.z
    }).map_err(|e| UriFormatError(e))?;
    
    let bytes = client
        .get(uri)
        .send().await.map_err(|e| ReqwestError(e))?
        .error_for_status().map_err(|e| ReqwestError(e))?
        .bytes().await.map_err(|e| ReqwestError(e))?;

    let image = Image::decode(&encoding, &bytes[..])
        .map_err(|e| ImageDecodeError(e))?;

    let preview = crate::preview::make_preview(&image, r.min_val, r.max_val)
        .ok_or(PreviewGenerateError)?;
    
    Ok(warp::http::Response::builder()
        .header("Content-Type", "image/png")
        .body(preview.data))
}

pub async fn run() {
    let routes = warp::get()
        .and(warp::query::<PreviewRequest>())
        .and_then(hello_world);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await
}
