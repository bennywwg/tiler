use serde::{Serialize, Deserialize};
use crate::image::{Image, ImageEncoding};
use glam::*;
use core::time::Duration;
use crate::uri_format::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub tile_uri_format: String,
    pub encoding: Option::<ImageEncoding>
}

#[derive(Serialize, Deserialize, Debug)]
struct DatasetProvider {
    tile_uri_format: String,
    encoding: ImageEncoding
}

fn format_tile_string(template: &str, coord: IVec3) -> Result<String, String> {
    crate::uri_fmt!(&template, {
        "x" => coord.x,
        "y" => coord.y,
        "z" => coord.z
    })
}

impl DatasetProvider {
    fn get_resource_uri(&self, coord: IVec3) -> String {
        match format_tile_string(self.tile_uri_format.as_str(), coord) {
            Ok(str) => str,
            Err(_) => panic!("format string is expected to be valid")
        }            
    }
    pub async fn load_resource(&self, coord: IVec3) -> Result<Image, String> {
        reqwest::Client::builder()
        .timeout(Duration::from_secs(999))
        .build().unwrap()
        .get(self.get_resource_uri(coord)).send().await
        .or(Err("abc".to_string()))?
        .bytes().await.or(Err("load body bytes failed".to_string()))
        .and_then(|bytes| Image::decode(&self.encoding, &bytes[..]))
    }
}