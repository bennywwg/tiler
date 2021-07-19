use iron::prelude::*;
use iron::{typemap, AfterMiddleware, BeforeMiddleware};
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
    let mut config: Option<PreviewRequest> = None;
    if let Some(query_str) = r.url.query() {
        config = Some(serde_json::from_str::<PreviewRequest>(&urlencoding::decode(query_str).unwrap()).unwrap());
        println!("{:?}", config);
    }
    
    match config {
        Some(reqConf) => {
            let body = reqwest::blocking::get("https://spkit.org/datasets/remapped/222_144_000.hgt").unwrap().bytes().unwrap();
    
            let image = Image::decode(ImageEncoding::srtm(), &body[..]).unwrap();
            let image_mapped = crate::preview::make_preview(&image, reqConf.min_val, reqConf.max_val).unwrap();
            Ok(Response::with((iron::status::Ok, "image/png", image_mapped.data)))
        },
        None => {
            Ok(Response::with((iron::status::Ok, "text/plain", "failed somehow")))
        }
    }

    //Ok(Response::with((iron::status::Ok, "text/plain", "hello")))
}

pub fn run() {
    let mut chain = Chain::new(hello_world);
    // chain.link_before(ResponseTime);
    // chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000");
}