extern crate image;
use image::Rgb;
extern crate num_complex;

use crate::raytracer::ray::*;
use na::Vector3;

#[typetag::serde(tag = "type")]
pub trait Shape3D
{

    fn ray_closest_intersections (&self, ray: &Ray) -> Option<(Vector3<f64>, f64)>;

    fn get_color (&self) -> [u8; 3];
}