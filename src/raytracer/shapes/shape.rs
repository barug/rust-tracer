extern crate image;
extern crate num_complex;

use crate::raytracer::ray::*;
use crate::raytracer::Intersection;

#[typetag::serde(tag = "type")]
pub trait Shape3D
{
    // returns (intersection point, distance from camera to intersection, normal vector)
    fn ray_closest_intersections (&self, ray: &Ray) -> Option<Intersection>;

    fn get_color (&self) -> [u8; 3];
}