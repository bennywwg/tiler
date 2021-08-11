use serde::{Serialize, Deserialize};
use glam::*;
use crate::util::math::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tilespace {
    pub size: IVec2,
    pub offset: IVec2
}

impl Tilespace {
    pub fn get_covered_tiles(&self, pixel_bounds: Dabb2) -> Dabb2 {
        Dabb2::bounds(
            pixel_bounds.begin.floor_on_interval(self.size),
            pixel_bounds.end.floor_on_interval(self.size) + self.size
        ) / self.size
    }
    pub fn tile_pixels(&self, intput_coord: IVec2) -> Dabb2 {
        (Dabb2::cell(intput_coord) * self.size) + self.offset
    }
    pub fn tile_pixels_level(&self, input_coord: IVec3) -> Dabb2 {
        (Dabb2::cell(ivec2(input_coord.x, input_coord.y)) * (self.size * (1 << input_coord.z))) + self.offset
    }
}

pub trait TileURIProvider {
    fn get_resource_uri(&self, coord: IVec3) -> String;
}

pub fn format_tile_string(template: &str, coord: IVec3) -> Result<String, String> {
    crate::uri_fmt!(&template, {
        "x" => coord.x,
        "y" => coord.y,
        "z" => coord.z
    })
}