extern crate image;
use image::Rgb;
extern crate num_complex;

use crate::coordinates::Coordinates3D;
use crate::raytracer::line::*;

#[typetag::serde(tag = "type")]
pub trait Shape3D
{

    fn ray_closest_intersections (&self, ray: &Line) -> Option<(Coordinates3D, f64)>;

    fn get_color (&self) -> [u8; 3];
}