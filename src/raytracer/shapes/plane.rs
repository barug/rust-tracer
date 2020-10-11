use serde::{Serialize, Deserialize};
use na::Vector3;

use super::shape::*; 
use crate::raytracer::ray::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plane {
    pub origin: Vector3<f64>,
    pub normal_vec: Vector3<f64>,
    pub color: [u8; 3]
}

impl Plane {
    pub fn new(origin: Vector3<f64>, normal_vec: Vector3<f64>, color: [u8; 3]) -> Plane {
        Plane{ origin: origin, normal_vec: normal_vec, color: color}
    }
}

#[typetag::serde]
impl Shape3D for Plane {
    fn ray_closest_intersections (&self, ray: &Ray) -> Option<(Vector3<f64>, f64)> {
        let l_dot_n: f64 = ray.unit_vec.dot(&self.normal_vec);
        if l_dot_n != 0.0 {
            let dist = (&self.origin - &ray.origin).dot(&self.normal_vec) / l_dot_n;
            if dist < 0.0 {
                return None
            }
            let intersect = &ray.origin + &ray.unit_vec * dist;
            return Some((intersect, dist))
        }
        None
    }

    fn get_color (&self) -> [u8; 3] {
        return self.color;
    }
}