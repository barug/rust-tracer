extern crate image;
use image::{RgbImage, RgbaImage, Rgb, Rgba, GenericImage, GenericImageView, Pixel};
extern crate num_complex;

mod drawing_2d;
mod coordinates;
mod raytracer;
use crate::raytracer::*;

use std::fs;
use std::fs::File;
use std::io::prelude::*;


fn save_scene(scene: &Scene, filename: String) -> std::io::Result<()> {
    let serialized = serde_yaml::to_string(&scene).unwrap();

    let mut file = fs::File::create(filename)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let imgx = 800;
    let imgy = 800;
    let mut imgbuf = RgbImage::new(imgx, imgy);

    let conf = fs::read_to_string("scene.yml")?;


    let scene: Scene = serde_yaml::from_str(&conf).unwrap();;
    

    scene.raytracing(&mut imgbuf);

    // Save the image as test.png”, the format is deduced from the path
    imgbuf.save("test.png").unwrap();
    Ok(())
}

