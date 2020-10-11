use serde::{Serialize, Deserialize};

use crate::coordinates::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Coordinates3D,
    pub unit_vec: Coordinates3D,
    pub inverse: Coordinates3D,
    pub sign: [usize; 3]
}

impl Ray {
    // Calculate unit vector of line form origin and other point
    pub fn new_from_points(origin: &Coordinates3D, other: &Coordinates3D) -> Ray {
        let d: f64 = origin.dist(other);
        let unit = Coordinates3D::new(
            (other.x - origin.x) / d,
            (other.y - origin.y) / d,
            (other.z - origin.z) / d,
        );
        let inverse = 1.0 / &unit;
        let sign = [
            (inverse.x < 0.0) as usize,
            (inverse.y < 0.0) as usize,
            (inverse.z < 0.0) as usize,
        ];
        Ray {origin : (*origin).clone(), unit_vec: unit, inverse: inverse, sign: sign}
    }
}