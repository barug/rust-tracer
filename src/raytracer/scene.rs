extern crate image;
use image::{Pixel};

use super::shapes::*;


pub struct Scene<P>
    where P: Pixel
{
    pub shapes: Vec<Box<dyn Shape3D<P>>>
}

impl<P> Scene<P>
    where P: Pixel
{
    pub fn new() -> Scene<P> {
        Scene {
            shapes: Vec::new()
        }
    }

    pub fn push_shape(&mut self, shape: Box<dyn Shape3D<P>>) {
        self.shapes.push(shape);
    }
}