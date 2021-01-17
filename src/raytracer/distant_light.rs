use serde::{Serialize, Deserialize};
use na::Vector3;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DistantLight {
    pub direction: Vector3<f64>
}
