extern crate image;
use std::ops::Mul;

use serde::{Serialize, Deserialize};
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;


use super::{DistantLight, intersection, shapes::*};
use super::camera::*;
use super::ray::*;
use super::utils::*;
use intersection::Intersection;


use na::{Vector3, Rotation3};
use na::geometry::Rotation;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Box<dyn Shape3D + Sync>>,
    pub lights: Vec<DistantLight>
}

impl Scene {
    pub fn render_scene(& self, dimx: u32, dimy: u32) -> Vec<u16> {
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

        let pixels: Vec<u16> = (0..num_pix)
            .into_par_iter() // create parralel iterator with rayon
            .progress_count(num_pix as u64) 
            .flat_map(
                |i| {
                    let pi_x: u32 = i % dimx;
                    let pi_y: u32 = i / dimx;
                    
                    let pos_pix = &P_1_1 + &q_x * pi_x as f64 - &q_y * pi_y as f64;
                    let ray: Ray = Ray::new_from_points(&self.camera.cam_pos, &pos_pix);
                        
                    let shaded_color = self.trace_ray(ray, 2);
                    // <[u16;3]>::from(shaded_color)
                    [
                        (shaded_color.x * 65535.0) as u16, 
                        (shaded_color.y * 65535.0) as u16, 
                        (shaded_color.z * 65535.0) as u16
                    ]
                }
            ).collect();
        pixels
    }


    fn trace_ray(&self, ray: Ray, depth: u8) -> Vector3<f64> {
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
            let distant_light_shading: Vector3<f64> = self.distant_light_shading(intersection);
            let global_illumination = self.path_tracing(&intersection, depth);
            let reflection_shading: Vector3<f64> = self.reflection_shading(&intersection, &ray, depth);

            return intersection.shape.get_emissive_color() + distant_light_shading + global_illumination + reflection_shading
        }
        
        Vector3::<f64>::from_element(0_f64)
    }

    fn reflection_shading(&self, intersection: &Intersection, ray: &Ray, depth: u8) -> Vector3<f64> {
        let reflectivity = intersection.shape.get_reflectivity();

        if reflectivity > 0.0 && depth > 0 {
            let reflection_vector = ray.unit_vec - 2.0 * intersection.normal.dot(&ray.unit_vec) * intersection.normal;
            let reflection_origine = &intersection.biased_location;
            let reflection_ray = Ray::new_from_origine_and_direction(&reflection_origine, &reflection_vector);

            reflectivity * self.trace_ray(reflection_ray, depth - 1)
        } else {
            Vector3::<f64>::from_element(0_f64)
        }
    }

    fn distant_light_shading(&self, intersection: &Intersection) -> Vector3<f64> {

        let albedo =  intersection.shape.get_albedo();
        
        let diffuse_reflection: f64 = self.lights.iter()
            .map( |light| {
                let origine = &intersection.biased_location;

                let light_direction_inverse = -light.direction;
                let reverse_lightray = Ray::new_from_origine_and_direction(&origine, &light_direction_inverse);

                let is_obstructed = self.shapes.iter()
                    .any(
                        |shape| shape.ray_closest_intersections(&reverse_lightray).is_some()
                    );
                if is_obstructed {
                    return 0.0
                }

                let angle = light_direction_inverse.angle(&intersection.normal);
                if angle < (std::f64::consts::PI / 2.0) {
                    albedo / std::f64::consts::PI * light.intensity * angle.cos()
                } else {
                    0.0
                }
            }).sum::<f64>();
        let shading_coefficient: f64 = diffuse_reflection;
        if shading_coefficient > 0.0 {
            let color = intersection.shape.get_color(); 
            let shaded_color = color * shading_coefficient;
            return shaded_color;
        }
        Vector3::<f64>::from_element(0_f64)
    }

    fn path_tracing(&self, intersection: &Intersection, depth: u8) -> Vector3<f64> {
        if depth == 0 {
            return Vector3::<f64>::from_element(0_f64)
        }
        let normal_coordinates_system = create_coordinate_system_from_up_vector(&intersection.normal);
        let rotation = Rotation3::from_basis_unchecked(&normal_coordinates_system);
        let nbr_of_samples = 200;

        let global_lighting_sum = (0..nbr_of_samples)
            .into_iter()
            .map(
                |_| {
                    let rand_direction = rotation * uniform_sampling_hemisphere();
                    let ray: Ray = Ray::new_from_origine_and_direction(&intersection.biased_location, &rand_direction);
                    let ray_angle = rand_direction.angle(&intersection.normal);
                    let indirect_light_color = self.trace_ray(ray, depth - 1);
                    indirect_light_color * ray_angle.cos()
                }
            ).sum::<Vector3<f64>>();
            // probability for a ray is 1 / 2 * pi -> divided by 1 / 2 pi ->  2 * pi
        
        let global_lighting = global_lighting_sum / nbr_of_samples as f64 * 2.0 * intersection.shape.get_albedo();
        let global_illumination_shading = global_lighting.component_mul(&intersection.shape.get_color());

        global_illumination_shading
    }

    pub fn push_shape(&mut self, shape: Box<dyn Shape3D + Sync>) {
        self.shapes.push(shape);
    }
}