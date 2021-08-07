use core::time::Duration;
use serde::{Serialize, Deserialize};
use glam::*;
use warp::*;

use crate::image::*;
use crate::serde_json_warp;
use crate::config::*;

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
    let codec = r.decode_info.ok_or(reject())?;

    let dp
    =DatasetProvider::create(r.tile_uri_format.as_str(), codec)
    .map_err(|_| reject())?;

    let image
    =dp.load_resource(r.coord).await
    .ok_or(PreviewGenerateError)?;

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
