extern crate image;
use image::{RgbImage, RgbaImage, Rgb, Rgba, GenericImage, GenericImageView, Pixel};
extern crate num_complex;

mod drawing_2d;
mod coordinates;
mod raytracer;
use crate::raytracer::*;

use std::fs;


fn main() {
    let imgx = 800;
    let imgy = 800;
    let mut imgbuf = RgbImage::new(imgx, imgy);

    let conf = fs::read_to_string("scene.yml")
        .expect("Something went wrong reading the configuration file");

    let mut scene: Scene = serde_yaml::from_str(&conf).unwrap();
    scene.push_shape(std::boxed::Box::new(
        shapes::Cuboid::new(
            coordinates::Coordinates3D::new(0.0, 0.0, 0.0),
            [
                coordinates::Coordinates3D::new(-1.0, -1.0, -1.0),
                coordinates::Coordinates3D::new(1.0, 1.0, 1.0)
            ],
            [100, 0, 100]
        )
    )
    );
    

    scene.raytracing(&mut imgbuf);

    // Save the image as test.png”, the format is deduced from the path
    imgbuf.save("test.png").unwrap();
}

