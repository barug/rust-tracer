extern crate image;
use image::{RgbImage, RgbaImage, Rgb, Rgba, GenericImage, GenericImageView, Pixel, ImageBuffer};
extern crate num_complex;
extern crate nalgebra as na;
use na::Vector3;

mod drawing_2d;
mod coordinates;
mod raytracer;
use crate::raytracer::*;
use crate::raytracer::camera::*;

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
    let mut imgbuf = ImageBuffer::<Rgb<u16>, Vec<u16>>::new(imgx, imgy);

    let conf = fs::read_to_string("scene.yml")?;
    let scene: Scene = serde_yaml::from_str(&conf).unwrap();
    
    scene.render_scene(&mut imgbuf);

    // Save the image as test.png”, the format is deduced from the path
    imgbuf.save("test3.png").unwrap();
    Ok(())
}

