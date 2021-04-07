use na::Vector3;

use super::shape::Shape3D;


pub struct Intersection<'a> {
    pub location: Vector3<f64>,
    pub distance: f64,
    pub normal: Vector3<f64>,
    pub shape: &'a dyn Shape3D
    // pub color: [u8; 3]
}

impl<'a> Intersection<'a> {
    pub fn new(location: Vector3<f64>, distance: f64, normal: Vector3<f64>, shape: &'a dyn Shape3D) -> Intersection<'a> {
        Intersection{location: location, distance: distance, normal: normal, shape: shape}
    }
}