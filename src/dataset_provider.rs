// use serde::{Serialize, Deserialize};
// use crate::image::{Image, ImageCodec};
// use glam::*;
// use core::time::Duration;
// use crate::util::math::*;

// pub trait DatasetProvider {
//     pub async fn resource_exists(&self, coord: IVec3) -> bool;
//     pub async fn load_resource(&self, coord: IVec3) -> Option<Image>;
// }