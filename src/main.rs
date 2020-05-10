extern crate image;
use image::{RgbImage, RgbaImage, Rgb, Rgba, GenericImage, GenericImageView, Pixel};
extern crate num_complex;
// use serde::{Serialize, Deserialize};

mod drawing_2d;
mod coordinates;
mod raytracer;
use crate::coordinates::*;
use crate::raytracer::*;

use serde::{Serialize, Deserialize};

use std::fs;


fn main() {
    let imgx = 800;
    let imgy = 800;
    let mut imgbuf = RgbImage::new(imgx, imgy);

    let conf =  fs::read_to_string("scene.yml")
    .expect("Something went wrong reading the configuration file");

    let scene: Scene = serde_yaml::from_str(&conf).unwrap();

    raytracer::raytracing(&mut imgbuf, scene);

    // Save the image as test.png”, the format is deduced from the path
    imgbuf.save("test.png").unwrap();
}

