extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};
use intersection::Intersection;
use serde::{Serialize, Deserialize, Serializer};

use super::{DistantLight, intersection, shapes::*};
use super::camera::*;
use super::ray::*;


use na::Vector3;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Box<dyn Shape3D>>,
    pub lights: Vec<DistantLight>
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
                if let Some(shaded_color) = self.apply_shading(intersection) {
                    img.put_pixel(pi_x, pi_y, Rgb(shaded_color));
                }
            }                
        }
    }

    fn apply_shading(&self, intersection: &Intersection) -> Option<[u8; 3]> {
        let shading_coefficient: f64 = &self.lights.iter()
            .map( |light| {
                let angle = light.direction.angle(&intersection.normal);
                if angle < (std::f64::consts::PI / 2.0) {
                    1.0 - (angle / (std::f64::consts::PI / 2.0))
                } else {
                    0.0
                }
            }).sum::<f64>() / self.lights.len() as f64;
        if shading_coefficient > 0.0 {
            let shaded_color: [u8; 3] = [
                (intersection.color[0] as f64 * shading_coefficient) as u8, 
                (intersection.color[1] as f64 * shading_coefficient) as u8,
                (intersection.color[2] as f64 * shading_coefficient) as u8
            ];
            return Some(shaded_color);
        }
        None
    }

    pub fn push_shape(&mut self, shape: Box<dyn Shape3D>) {
        self.shapes.push(shape);
    }
}