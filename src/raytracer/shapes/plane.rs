use serde::{Serialize, Deserialize};
use na::Vector3;

use super::shape::*; 
use crate::raytracer::ray::*;
use crate::raytracer::Intersection;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plane {
    pub origin: Vector3<f64>,
    pub normal_vec: Vector3<f64>,
    pub color: Vector3<u16>,
    albedo: f64
}

impl Plane {
    pub fn new(origin: Vector3<f64>, normal_vec: Vector3<f64>, color: Vector3<u16>, albedo: f64) -> Plane {
        Plane{ origin: origin, normal_vec: normal_vec, color: color, albedo: albedo}
    }
}

#[typetag::serde]
impl Shape3D for Plane {
    fn ray_closest_intersections (&self, ray: &Ray) -> Option<Intersection> {
        let l_dot_n: f64 = ray.unit_vec.dot(&self.normal_vec);
        if l_dot_n != 0.0 {
            let distance = (&self.origin - &ray.origin).dot(&self.normal_vec) / l_dot_n;
            if distance < 0.0 {
                return None
            }
            let location = &ray.origin + &ray.unit_vec * distance;
            let normal: Vector3<f64> = if ray.unit_vec.angle(&self.normal_vec) > std::f64::consts::PI / 2.0 {
                self.normal_vec.clone_owned()
            } else {
                - self.normal_vec.clone_owned()
            };
            // println!("{:?}", &ray.unit_vec);
            // println!("{:?}", ray.unit_vec.angle(&self.normal_vec));
            // println!("{:?}", normal);
            return Some(Intersection::new(location, distance, normal, self))
        }
        None
    }

    fn get_color (&self) -> Vector3<u16> {
        return self.color;
    }

    fn get_albedo (&self) -> f64 {
        return self.albedo;
    }
}