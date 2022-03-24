extern crate image;
extern crate num_complex;

use crate::raytracer::ray::*;
use crate::raytracer::Intersection;

use na::Vector3;

#[typetag::serde(tag = "type")]
pub trait Shape3D : Sync + std::fmt::Debug
{
    // returns (intersection point, distance from camera to intersection, normal vector)
    fn ray_closest_intersections (&self, ray: &Ray) -> Option<Intersection>;

    fn get_color (&self) -> Vector3<f64>;

    fn get_emissive_color(&self) -> Vector3<f64>;

    fn get_albedo(&self) -> f64;

    fn get_reflectivity(&self) -> f64;
}