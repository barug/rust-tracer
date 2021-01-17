use serde::{Serialize, Deserialize};
use na::Vector3;

// use crate::coordinates::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub unit_vec: Vector3<f64>,
    pub inverse: Vector3<f64>,
    pub sign: [usize; 3]
}

impl Ray {
    // Calculate unit vector of line form origin and other point
    pub fn new_from_points(origin: &Vector3<f64>, other: &Vector3<f64>) -> Ray {
        let d: f64 = (origin - other).norm();
        let unit: Vector3<f64> = ((other - origin) / d).normalize();
        let inverse: Vector3<f64> = Vector3::new(
            1.0 / unit.x,
            1.0 / unit.y,
            1.0 / unit.z,
        );        
        let sign = [
            (inverse.x < 0.0) as usize,
            (inverse.y < 0.0) as usize,
            (inverse.z < 0.0) as usize,
        ];
        Ray {origin : (*origin).clone(), unit_vec: unit, inverse: inverse, sign: sign}
    }
}