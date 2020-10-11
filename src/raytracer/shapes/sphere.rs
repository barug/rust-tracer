use image::Rgb;
use serde::{Serialize, Deserialize};

use super::shape::*; 
use crate::raytracer::ray::*;
use na::Vector3;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub centre: Vector3<f64>,
    pub r: f64,
    pub color: [u8; 3]
}

impl Sphere {

    // pub fn new(x :f64, y: f64, z: f64, r: f64) -> Sphere {
    //     Sphere {
    //         centre : Coordinates3D::new(x, y, z),
    //         r: r
    //     }
    // }

    pub fn new(centre: Vector3<f64>, r: f64, color: [u8; 3]) -> Sphere {
        Sphere {
            centre : centre,
            r: r,
            color: color
        }
    }
}

#[typetag::serde]
impl Shape3D for Sphere {

    fn ray_closest_intersections (&self, ray: &Ray) -> Option<(Vector3<f64>, f64)> {
        let or_sub_centr = &ray.origin - &self.centre;
        let discriminant: f64 = ray.unit_vec.dot(&or_sub_centr).powi(2) - (or_sub_centr.norm().powi(2) - self.r.powi(2));

        if discriminant < 0.0 {
            return None
        }
        else if discriminant == 0.0 {
            let dist: f64 = - ray.unit_vec.dot(&or_sub_centr);
            if dist < 0.0 { 
                return None
            }
        } else {
            let dist1: f64 = - ray.unit_vec.dot(&or_sub_centr) + discriminant.sqrt();            
            let dist2: f64 = - ray.unit_vec.dot(&or_sub_centr) - discriminant.sqrt();

            if dist1 >= 0.0 && (dist2 < 0.0 || dist2 > dist1) {
                let intersect1: Vector3<f64> = &ray.origin + &ray.unit_vec * dist1;
                return Some((intersect1, dist1))
            } else if dist2 >= 0.0 {
                let intersect2: Vector3<f64> = &ray.origin + &ray.unit_vec * dist2;
                return Some((intersect2, dist2))
            }
        }
        return None;
    }

    fn get_color (&self) -> [u8; 3] {
        return self.color;
    }
}
