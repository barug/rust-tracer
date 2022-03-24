use serde::{Serialize, Deserialize};

use super::shape::*; 
use crate::raytracer::ray::*;
use crate::raytracer::Intersection;

use na::Vector3;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub centre: Vector3<f64>,
    pub r: f64,
    pub color: Vector3<f64>,
    pub emissive_color: Vector3<f64>,
    pub albedo: f64,
    pub reflectivity: f64
}

impl Sphere {
    pub fn new(
        centre: Vector3<f64>, 
        r: f64, 
        color: Vector3<f64>, 
        emissive_color: Vector3<f64>, 
        albedo: f64,
        reflectivity: f64
    ) -> Sphere {
        Sphere {centre, r, color, emissive_color, albedo, reflectivity}
    }
}

#[typetag::serde]
impl Shape3D for Sphere {

    fn ray_closest_intersections (&self, ray: &Ray) -> Option<Intersection> {
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
                let location: Vector3<f64> = &ray.origin + &ray.unit_vec * dist1;
                let normal: Vector3<f64> = (&location - &self.centre).normalize(); 
                return Some(Intersection::new(location, dist1, normal, self))
            } else if dist2 >= 0.0 {
                let location: Vector3<f64> = &ray.origin + &ray.unit_vec * dist2;
                let normal: Vector3<f64> = (&location - &self.centre).normalize();
                return Some(Intersection::new(location, dist2, normal, self))
            }
        }
        return None;
    }

    fn get_color (&self) -> Vector3<f64> {
        return self.color;
    }

    fn get_emissive_color (&self) -> Vector3<f64> {
        return self.emissive_color;
    }

    fn get_albedo (&self) -> f64 {
        return self.albedo;
    }

    fn get_reflectivity(&self) -> f64 {
        return self.reflectivity;
    }
}
