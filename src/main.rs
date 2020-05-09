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
    // imgbuf.put_pixel(x: u32, y: u32, pixel: P)

    let s = Box::new(
        Sphere::new(
            Coordinates3D::new(0.0, 0.0, 8.0),
            2.0,
            [200, 200, 200]
        )
    );

    
    let s2 = Box::new(
        Sphere::new(
            Coordinates3D::new(3.0, 0.0, 7.0),
            2.0,
            [200, 100, 200]
        )
    );

    let s3 = Box::new(
        Plane::new(
            Coordinates3D::new(0.0, -1.0, 0.0),
            Coordinates3D::new(0.0, 1.0, 0.0),
            [200, 200, 100]
        )
    );

    
    let mut scene: Scene = Scene::new();
    scene.push_shape(s);
    scene.push_shape(s2);
    scene.push_shape(s3);

    raytracer::raytracing(&mut imgbuf, scene);

    // Save the image as test.png”, the format is deduced from the path
    imgbuf.save("test.png").unwrap();
}

