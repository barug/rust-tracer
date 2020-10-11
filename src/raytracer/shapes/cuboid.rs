use image::Rgb;
use serde::{Serialize, Deserialize};

use std::cmp::max;
use std::cmp::min;


use super::shape::*; 
use crate::coordinates::Coordinates3D;
use crate::raytracer::ray::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cuboid {
    pub position: Coordinates3D,
    pub bounds: [Coordinates3D; 2], 
    pub color: [u8; 3]
}

impl Cuboid {
    pub fn new(position: Coordinates3D, bounds: [Coordinates3D; 2], color: [u8; 3]) -> Cuboid {
        Cuboid {
            position: position,
            bounds : bounds,
            color: color
        }
    }
}

#[typetag::serde]
impl Shape3D for Cuboid {

    // Bouding box AABB algorithm such as seen at :
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection
    // saved in doc folder in case of dead link
    fn ray_closest_intersections (&self, ray: &Ray) -> Option<(Coordinates3D, f64)> {

        let translated_origin: Coordinates3D = &ray.origin - &self.position;

        let txmin: f64 = (self.bounds[ray.sign[0]].x - translated_origin.x) * ray.inverse.x;
        let txmax: f64 = (self.bounds[1-ray.sign[0]].x - translated_origin.x) * ray.inverse.x; 
        let tymin: f64 = (self.bounds[ray.sign[1]].y - translated_origin.y) * ray.inverse.y; 
        let tymax: f64 = (self.bounds[1-ray.sign[1]].y - translated_origin.y) * ray.inverse.y; 
    
        if (txmin > tymax) || (tymin > txmax) {
            return None; 
        }

        let txymin: f64 = txmin.max(tymin);
        let txymax: f64 = txmax.min(tymax);

        let tzmin: f64 = (self.bounds[ray.sign[2]].z - translated_origin.z) * ray.inverse.z; 
        let tzmax: f64 = (self.bounds[1-ray.sign[2]].z - translated_origin.z) * ray.inverse.z; 

        if (txymin > tzmax) || (tzmin > txymax) {
            return None;
        }

        let txyzmin: f64 = txymin.max(tzmin);
        let txyzmax: f64 = txymax.min(tzmax);

 
        let dist = txyzmin;
        if txyzmin > txyzmax {
            return None
        }
        if dist < 0.0 {
            return None
        }

        let intersection = &ray.origin + &ray.unit_vec * dist;
        return Some((intersection, dist));
    }

    fn get_color (&self) -> [u8; 3] {
        return self.color;
    }
}
