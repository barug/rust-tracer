extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};
extern crate num_complex;

mod drawing_2d;
mod coordinates;
mod raytracer;
use crate::coordinates::*;
use crate::raytracer::*;


fn main() {
    let imgx = 800;
    let imgy = 800;

    let mut imgbuf = RgbImage::new(imgx, imgy);

    let pix: Rgb<u8> = Rgb([200, 200, 200]);
    let s = Box::new(
        Sphere::new(
            Coordinates3D::new(0.0, 0.0, 8.0),
            2.0,
            pix
        )
    );
    
    let mut scene: Scene<Rgb<u8>> = Scene::new();
    scene.push_shape(s);

    raytracer::raytracing(&mut imgbuf, scene);

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("test.png").unwrap();
}

