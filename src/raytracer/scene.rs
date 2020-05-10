extern crate image;
use serde::{Serialize, Deserialize, Serializer};

use super::shapes::*;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub shapes: Vec<Box<dyn Shape3D>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            shapes: Vec::new()
        }
    }

    pub fn push_shape(&mut self, shape: Box<dyn Shape3D>) {
        self.shapes.push(shape);
    }
}