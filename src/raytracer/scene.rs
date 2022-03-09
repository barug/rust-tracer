extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel, ImageBuffer};
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
    pub fn render_scene(& self, img: &mut ImageBuffer::<Rgb<u16>, Vec<u16>>) {
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
                
            let result = self.trace_ray(ray, 4);
            if let Some(shaded_color) = result {
                img.put_pixel(pi_x, pi_y, Rgb::<u16>(shaded_color.into()));
            }
        }
    }


    fn trace_ray(&self, ray: Ray, depth: u8) -> Option<Vector3<u16>> {
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
            let diffuse_shading: Vector3<u16> = self.diffuse_shading(intersection).unwrap_or(Vector3::<u16>::from_element(0_u16));

            let reflection_shading: Vector3<u16> = if depth > 0 {
                let reflection_vector = ray.unit_vec - 2.0 * intersection.normal.dot(&ray.unit_vec) * intersection.normal;
                let reflection_origine = &intersection.location + 0.01 * &intersection.normal;
                let reflection_ray = Ray::new_from_origine_and_direction(&reflection_origine, &reflection_vector);

                self.trace_ray(reflection_ray, depth - 1).unwrap_or(Vector3::<u16>::from_element(0_u16))
            } else {
                Vector3::<u16>::from_element(0_u16)
            };

            return Some(diffuse_shading + reflection_shading / 6)
        }
        
        None
    }


    fn diffuse_shading(&self, intersection: &Intersection) -> Option<Vector3<u16>> {

        let albedo =  intersection.shape.get_albedo();
        let ambiant_light = 0.2;
        let diffuse_reflection: f64 = self.lights.iter()
            .map( |light| {
                let origine = &intersection.location + 0.01 * &intersection.normal;
                let reverse_lightray = Ray::new_from_origine_and_direction(&origine, &light.direction);

                let is_obstructed = self.shapes.iter()
                    .any(
                        |shape| shape.ray_closest_intersections(&reverse_lightray).is_some()
                    );
                if is_obstructed {
                    return 0.0
                }

                let angle = light.direction.angle(&intersection.normal);
                if angle < (std::f64::consts::PI / 2.0) {
                    // 1.0 - (angle / (std::f64::consts::PI / 2.0))
                    albedo / std::f64::consts::PI * light.intensity * angle.cos()
                } else {
                    0.0
                }
            }).sum::<f64>();
        let shading_coefficient: f64 = ambiant_light + diffuse_reflection;
        if shading_coefficient > 0.0 {
            let color = intersection.shape.get_color(); 
            let shaded_color: Vector3<u16> = [
                (color[0] as f64 * shading_coefficient).min(65535.0) as u16, 
                (color[1] as f64 * shading_coefficient).min(65535.0) as u16,
                (color[2] as f64 * shading_coefficient).min(65535.0) as u16
            ].into();
            return Some(shaded_color);
        }
        None
    }

    pub fn push_shape(&mut self, shape: Box<dyn Shape3D>) {
        self.shapes.push(shape);
    }
}