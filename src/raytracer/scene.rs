extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};
use serde::{Serialize, Deserialize, Serializer};

use super::{intersection, shapes::*};
use super::camera::*;
use super::ray::*;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Box<dyn Shape3D>>
}

impl Scene {
    pub fn raytracing(& self, img: &mut RgbImage) {
        let (dimx, dimy): (u32, u32) = img.dimensions();
        let num_pix: u32             = dimx * dimy; 
        
        let fov = std::f64::consts::PI / 4.0;

        let t = &self.camera.cam_orient;
        let w = &self.camera.up_vec;
        let b = w.cross(&t);
        let v = t.cross(&b);
        
        let g_x: f64 = (fov / 2.0).tan();
        let g_y: f64 = g_x * (dimy as f64 / dimx as f64);

        
        let P_1_1 = &self.camera.cam_pos + t - g_x * &b + g_y * &v;
        
        let q_x = ((2.0 * g_x) / (dimx as f64 - 1.0)) * &b;
        let q_y = ((2.0 * g_y) / (dimy as f64 - 1.0)) * &v;

        
        for i in 0..num_pix {
            let pi_x: u32 = i % dimx;
            let pi_y: u32 = i / dimx;
            
            let pos_pix = &P_1_1 + &q_x * pi_x as f64 - &q_y * pi_y as f64;
            let ray: Ray = Ray::new_from_points(&self.camera.cam_pos, &pos_pix);
    
            let result = self.shapes
                .iter()
                .flat_map(|shape| shape.ray_closest_intersections(&ray))
                .min_by(
                    |intersection_1, intersection_2| {
                        intersection_1.distance
                            .partial_cmp(&intersection_2.distance)
                            .unwrap()
                    }
                );
            if let Some(intersection) = &result {
                img.put_pixel(pi_x, pi_y, Rgb(intersection.color));
            }                
        }
    }

    pub fn push_shape(&mut self, shape: Box<dyn Shape3D>) {
        self.shapes.push(shape);
    }
}