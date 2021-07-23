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
    let query_str = match r.url.query() {
        Some(query_str) => query_str,
        None => return Ok(Response::with((iron::status::BadRequest, "text/plain", "somehow getting the query string failed?")))
    };

    let decoded_query_str = match urlencoding::decode(query_str) {
        Ok(decoded) => decoded,
        Err(_) => return Ok(Response::with((iron::status::BadRequest, "text/plain", "query string encoding was malformed")))
    };

    let conf = match serde_json::from_str::<PreviewRequest>(&decoded_query_str) {
        Ok(config) => config,
        Err(_) => return Ok(Response::with((iron::status::BadRequest, "text/plain", "failed to parse preview request struct")))
    };
    
    let image_encoding = match conf.config.encoding {
        Some(encoding) => encoding,
        None => return Ok(Response::with((iron::status::BadRequest, "text/plain", "dataset config does not have an explicit encoding")))
    };

    let req_res = match reqwest::blocking::get("https://spkit.org/datasets/remapped/222_144_000.hgt") {
        Ok(res) => res,
        Err(_) => return Ok(Response::with((iron::status::BadGateway, "text/plain", "requesting resource from server failed")))
    };

    let body = match req_res.bytes() {
        Ok(body) => body,
        Err(_) => return Ok(Response::with((iron::status::BadGateway, "text/plain", "requesting resource body from server failed")))
    };

    let image = match Image::decode(image_encoding, &body[..]) {
        Ok(image) => image,
        Err(_) => return Ok(Response::with((iron::status::InternalServerError, "text/plain", "decoding the image bytes failed")))
    };

    return match crate::preview::make_preview(&image, conf.min_val, conf.max_val) {
        Some(png_bytes) => Ok(Response::with((iron::status::Ok, "image/png", png_bytes.data))),
        None => Ok(Response::with((iron::status::Ok, "text/plain", "generating preview failed")))
    };
}

pub fn run() {
    let chain = Chain::new(hello_world);
    // chain.link_before(ResponseTime);
    // chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}