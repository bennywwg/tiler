use core::time::Duration;
use iron::prelude::*;
use serde::{Serialize, Deserialize};
use glam::*;

use crate::image::*;
use crate::config::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviewRequest {
    pub config: DatasetConfig,
    pub coord: IVec3,
    pub min_val: f32,
    pub max_val: f32
}

fn hello_world(r: &mut Request) -> IronResult<Response> {
    let a = r.url.query().ok_or((iron::status::BadRequest, "text/plain", "somehow getting the query string failed"))
    .and_then(|s| urlencoding::decode(s).or(Err((iron::status::BadRequest, "text/plain", "query string encoding was malformed"))))
    .and_then(|s| serde_json::from_str::<PreviewRequest>(&s).or(Err((iron::status::BadRequest, "text/plain", "failed to parse preview request struct"))));

    let preview_request = match a {
        Ok(c) => c,
        Err(m) => return Ok(Response::with(m))
    };

    let encoding = preview_request.config.encoding.unwrap_or(ImageEncoding::srtm());

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(999))
        .build().unwrap();

    let b = client.get("https://spkit.org/datasets/remapped/222_144_000.hgt").send()
            .or(Err((iron::status::BadGateway, "text/plain", "requesting resource from server failed")))
        .and_then(|r| r.bytes().or(Err((iron::status::BadGateway, "text/plain", "requesting resource body from server failed"))))
        .and_then(|i| Image::decode(encoding, &i[..]).or(Err((iron::status::InternalServerError, "text/plain", "decoding the image bytes failed"))));

    let image = match b {
        Ok(i) => i,
        Err(m) => return Ok(Response::with(m))
    };

    let png_mime: iron::mime::Mime = "image/png".parse().unwrap();

    return match crate::preview::make_preview(&image, preview_request.min_val, preview_request.max_val) {
        Some(png_bytes) => Ok(Response::with((iron::status::Ok, png_mime, png_bytes.data))),
        None => Ok(Response::with((iron::status::Ok, "text/plain", "generating preview failed")))
    };


    // let req_res = match reqwest::blocking::get("https://spkit.org/datasets/remapped/222_144_000.hgt") {
    //     Ok(res) => res,
    //     Err(_) => return Ok(Response::with((iron::status::BadGateway, "text/plain", "requesting resource from server failed")))
    // };

    // let body = match req_res.bytes() {
    //     Ok(body) => body,
    //     Err(_) => return Ok(Response::with((iron::status::BadGateway, "text/plain", "requesting resource body from server failed")))
    // };



    // let d = c.and_then(|c| if c.config.encoding.is_some() { Ok(c) } else { Err((iron::status::BadRequest, "text/plain", "dataset config does not have an explicit encoding")) }); 

    // let query_str = match r.url.query() {
    //     Some(query_str) => query_str,
    //     None => return Ok(Response::with((iron::status::BadRequest, "text/plain", "somehow getting the query string failed?")))
    // };

    // let decoded_query_str = match urlencoding::decode(query_str) {
    //     Ok(decoded) => decoded,
    //     Err(_) => return Ok(Response::with((iron::status::BadRequest, "text/plain", "query string encoding was malformed")))
    // };

    // let conf = match serde_json::from_str::<PreviewRequest>(&decoded_query_str) {
    //     Ok(config) => config,
    //     Err(_) => return Ok(Response::with((iron::status::BadRequest, "text/plain", "failed to parse preview request struct")))
    // };
    // 
    // let image_encoding = match conf.config.encoding {
    //     Some(encoding) => encoding,
    //     None => return Ok(Response::with((iron::status::BadRequest, "text/plain", "dataset config does not have an explicit encoding")))
    // };

}

pub fn run() {
    let chain = Chain::new(hello_world);
    // chain.link_before(ResponseTime);
    // chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}
