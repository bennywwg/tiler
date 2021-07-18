use serde::{Serialize, Deserialize};
use crate::image::ImageEncoding;



#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetConfig {
    pub tile_uri_format: String,
    pub encoding: Option::<ImageEncoding>
}