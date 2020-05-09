use image::Rgb;

use super::shape::*; 
use crate::coordinates::Coordinates3D;
use crate::raytracer::line::*;

pub struct Plane {
    origin: Coordinates3D,
    normal_vec: Coordinates3D,
    color: [u8; 3]
}

impl Plane {
    pub fn new(origin: Coordinates3D, normal_vec: Coordinates3D, color: [u8; 3]) -> Plane {
        Plane{ origin: origin, normal_vec: normal_vec, color: color}
    }
}

impl Shape3D for Plane {
    fn ray_closest_intersections (&self, ray: &Line) -> Option<(Coordinates3D, f64)> {
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