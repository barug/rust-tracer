use serde::{Serialize, Deserialize};

use crate::coordinates::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Line {
    pub origin: Coordinates3D,
    pub unit_vec: Coordinates3D
}

impl Line {
    // Calculate unit vector of line form origin and other point
    pub fn new_from_points(origin: &Coordinates3D, other: &Coordinates3D) -> Line {
        let d: f64 = origin.dist(other);
        let unit = Coordinates3D::new(
            (other.x - origin.x) / d,
            (other.y - origin.y) / d,
            (other.z - origin.z) / d,
        );
        Line {origin : (*origin).clone(), unit_vec: unit}
    }
}