pub mod config;
pub mod preview;
pub mod util;
pub mod image;
pub mod dataset_cache;

//use reqwest::*;
use crate::image::*;
use std::fs;
//use imageproc::*;
//use ::image as image_ext;

#[tokio::main]
async fn main() {
    let mut cache = dataset_cache::DatasetCache::new(10, 10);
    {
        let ree = cache.access("ree");
        println!("{:?}", ree);
    }
    let ree2 = cache.access("ree");

    //let mut aaa = cache.access_mut("ree");


    println!("{:?}", ree2);
    //println!("{:?}", aaa);

    return;



    let body = reqwest::get("https://spkit.org/datasets/remapped/222_144_000.hgt").await.unwrap()
        .bytes().await.unwrap();

    let image = Image::decode(ImageEncoding::srtm(), &body[..]).unwrap();


    // let mut buf = image::GrayImage::from_raw(1201, 1201, (0..body.len() / 2).map(|i| (body[i * 2] as u16) << 8 | (body[i * 2 + 1] as u16)).map(|p| image::Luma([p])).collect());

    //for pair in buf.pixels_mut().flat_map(|p| p.channels_mut()).zip((0..body.len() / 2).map(|i| (body[i * 2] as u16) << 8 | (body[i * 2 + 1] as u16))) {
    //    *pair.0 = (pair.1 / 2) as u8;
    //}

    //let before = Instant::now();

    //let inv_range = 1.0 / 200.0;

    //let mapped = imageproc::map::map_colors(&buf, |c| {
    //    let (r, g, b) = preview::transform_pixel(c[0] as i16, 0.0, inv_range);
    //    image::Rgb([r, g, b])
    //});

    //println!("Elapsed: {:?}", before.elapsed());

    //mapped.save("./image.png").unwrap();

    let image_mapped = preview::make_preview(&image, 0.0, 400.0).unwrap();

    // println!("body = {:?}", image_mapped.encoding);

    fs::write("./pog.png", image_mapped.data).unwrap();
}
