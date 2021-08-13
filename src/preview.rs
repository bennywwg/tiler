use glam::*;
use num::*;
use crate::image::*;
use ::image as image_ext;

pub fn color_map(scalar: f32) -> Vec3 {
    // From "Why we use bad color maps and what you can do about it" (Kenneth Moreland)
    // Page 5, Figure 8
    let colors = [
        vec3(  0.0,   0.0,   0.0) / 255.0,
        vec3(  0.0,  24.0, 168.0) / 255.0,
        vec3( 99.0,   0.0, 228.0) / 255.0,
        vec3(220.0,  20.0,  60.0) / 255.0,
        vec3(255.0, 117.0,  56.0) / 255.0,
        vec3(238.0, 210.0,  20.0) / 255.0,
        vec3(255.0, 255.0, 255.0) / 255.0
    ];
    let positions = [
        0.0,
        0.22,
        0.35,
        0.47,
        0.65,
        0.84,
        1.0
    ];
    assert!(positions.len() == colors.len());
    if scalar <= 0.0 {
        return colors[0];
    }
    if scalar >= 1.0 {
        return colors[colors.len() - 1];
    }
    for i in 0..colors.len() {
        if scalar >= positions[i] && scalar < positions[i + 1] {
            return colors[i].lerp(colors[i + 1], (scalar - positions[i]) / (positions[i + 1] - positions[i]));
        }
    }
    return colors[colors.len() - 1];
}

fn to_rgb_u8(color: Vec3) -> UVec3 {
    UVec3::new(
        clamp((color.x * 255.0) as u32, 0, 255),
        clamp((color.y * 255.0) as u32, 0, 255),
        clamp((color.z * 255.0) as u32, 0, 255)
    )
}

pub fn transform_pixel(val: i16, min: f32, inv_range: f32) -> (u8, u8, u8) {
    let scalar = (val as f32 - min) * inv_range;

    let colorized = to_rgb_u8(color_map(scalar as f32));

    (colorized.x as u8, colorized.y as u8, colorized.z as u8)
}

pub fn make_preview(image: &impl Image, min: f32, max: f32) -> Option<ImageOwned> {
    let data = image.backing();

    let mut res_data = vec![0u8; data.len() / 2 * 3];

    let inv_range = 1.0  / (max - min);
    for i in 0..(data.len() / 2) {
        let val: i16 = (data[i * 2] as i16) << 8 | (data[i * 2 + 1] as i16);
        
        let (r, g, b) = transform_pixel(val, min, inv_range);
        res_data[i * 3] = r;
        res_data[i * 3 + 1] = g;
        res_data[i * 3 + 2] = b;
    }

    let a = image_ext::RgbImage::from_raw(image.get_format().size.x as u32, image.get_format().size.y as u32, res_data)?;
    let d = image_ext::DynamicImage::ImageRgb8(a);

    let mut asdf = vec![];
    d.write_to(&mut asdf, image_ext::ImageOutputFormat::Png).ok()?;

    Some(ImageOwned{
        format: ImageFormat {
            encoding: PixelEncoding::color(),
            size: image.get_format().size
        },
        data: asdf
    })
}