use core::time::Duration;
use serde::{Serialize, Deserialize};
use glam::*;
use warp::*;

use crate::image::*;
use crate::serde_json_warp;

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviewRequest {
    pub tile_uri_format: String,
    pub decode_info: Option<ImageCodec>,
    pub coord: IVec3,
    pub range: Vec2
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

async fn get_preview(r: PreviewRequest) -> Result<impl warp::reply::Reply, warp::Rejection> {
    let client
    =reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .build().unwrap();

    let uri
    =crate::uri_fmt!(&r.tile_uri_format, {
        "x" => r.coord.x,
        "y" => r.coord.y,
        "z" => r.coord.z
    }).map_err(|e| UriFormatError(e))?;
    
    let bytes
    =client
    .get(uri)
    .send().await.map_err(|e| ReqwestError(e))?
    .error_for_status().map_err(|e| ReqwestError(e))?
    .bytes().await.map_err(|e| ReqwestError(e))?;

    let di = r.decode_info.ok_or(ImageDecodeError("Must provide a value for decode_info (for now)".into()))?;

    let image = Image::decode(di, &bytes[..]).map_err(|e| ImageDecodeError(e))?;

    let preview
    =crate::preview::make_preview(&image, r.range.x, r.range.y)
    .ok_or(PreviewGenerateError)?;
    
    Ok(
        warp::http::Response::builder()
        .header("Content-Type", "image/png")
        .body(preview.data)
    )
}

pub async fn run() {
    let routes
    =warp::get()
    .and(serde_json_warp::query::<PreviewRequest>())
    .and_then(get_preview);

    warp::serve(routes)
    .run(([127, 0, 0, 1], 3000))
    .await
}
