extern crate image;
use image::Pixel;
extern crate num_complex;

use crate::coordinates::*;
use crate::raytracer::line::*;

pub trait Shape3D<P> 
    where P: Pixel
{

    // Compute all intersection with a line, if any
    // fn line_intersections (&self, line: &Line) -> Option<Vec<(Coordinates3D, f64)>>;

    fn ray_closest_intersections (&self, ray: &Line) -> Option<(Coordinates3D, f64)>;

    // // Compute only closest intersection to line origin, if any
    // fn closest_line_interstection (&self, line: &Line) -> Option<(Coordinates3D, f64)>;

    fn get_color (&self) -> P;
}