use serde::{Serialize, Deserialize};
use na::Vector3;

use super::shape::*; 
use crate::raytracer::ray::*;
use crate::raytracer::Intersection;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cuboid {
    pub position: Vector3<f64>,
    pub bounds: [Vector3<f64>; 2], 
    pub color: [u8; 3]
}

impl Cuboid {
    pub fn new(position: Vector3<f64>, bounds: [Vector3<f64>; 2], color: [u8; 3]) -> Cuboid {
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
    fn ray_closest_intersections (&self, ray: &Ray) -> Option<Intersection> {

        let translated_origin: Vector3<f64> = &ray.origin - &self.position;

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

 
        let distance = txyzmin;
        if txyzmin > txyzmax {
            return None
        }
        if distance < 0.0 {
            return None
        }

        let location = &ray.origin + &ray.unit_vec * distance;

        // the normal is on the axis with the highest absolute value
        let mut normal: Vector3<f64> = Vector3::<f64>::zeros();
        normal[location.iamax()] = 1.0;

        return Some(Intersection::new(location, distance, normal, self.get_color()))
    }

    fn get_color (&self) -> [u8; 3] {
        return self.color;
    }
}
