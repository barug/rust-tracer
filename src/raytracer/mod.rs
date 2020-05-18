pub mod line;
pub mod shapes;
pub mod scene;

extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};

use crate::coordinates::Coordinates3D;
pub use self::shapes::*;
pub use self::line::*;
pub use self::scene::*;

pub fn raytracing(img: &mut RgbImage, scene: Scene) {
    let (dimx, dimy): (u32, u32) = img.dimensions();
    let num_pix: u32             = dimx * dimy; 
    
    let cam_orient: Coordinates3D = Coordinates3D::new(0.0, 0.0, 1.0);
    let cam_pos: Coordinates3D    = Coordinates3D::new(0.0 ,0.0 ,-7.0);
    let up_vec: Coordinates3D     = Coordinates3D::new(0.0 ,1.0 ,0.0);
    
    let dis_viewport: f64        = 1.0;
    let viewport_width: f64      = 1.0;

    let fov = std::f64::consts::PI / 4.0;

    let t = cam_orient;
    let w = up_vec;
    let b = w.cross(&t);
    let v = t.cross(&b);
    println!("b: {:?}", b);
    println!("v: {:?}", v);

    let g_x: f64 = (fov / 2.0).tan();
    let g_y: f64 = g_x * (dimy as f64 / dimx as f64);

    println!("g_x: {:?}, g_y: {:?}", g_x, g_y);

    let P_1_1 = &cam_pos + t - g_x * &b + g_y * &v;
    println!("P_1_1: {:?}", &P_1_1);
    
    let q_x = ((2.0 * g_x) / (dimx as f64 - 1.0)) * &b;
    let q_y = ((2.0 * g_y) / (dimy as f64 - 1.0)) * &v;

    println!("q_x: {:?}, q_y: {:?}", &q_x, &q_y);

    for i in 0..num_pix {
        let pi_x: u32 = i % dimx;
        let pi_y: u32 = i / dimx;
        
        let pos_pix = &P_1_1 + &q_x * pi_x as f64 - &q_y * pi_y as f64;
        let mut ray: Line = Line::new_from_points(&cam_pos, &pos_pix);
        ray.unit_vec = &ray.unit_vec / ray.unit_vec.norm();

        let mut closest: f64 = std::f64::INFINITY;

        for shape in &scene.shapes {
            let result = shape.ray_closest_intersections(&ray);
            if let Some(intersection) = &result {
                if intersection.1 < closest {
                    img.put_pixel(pi_x, pi_y, Rgb(shape.get_color()));
                    closest = intersection.1;
                }
            }
        }
    }
}