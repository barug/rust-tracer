use serde::{Serialize, Deserialize};
use na::Vector3;
// use crate::coordinates::*;

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub cam_orient: Vector3<f64>,
    pub cam_pos: Vector3<f64>,
    pub up_vec: Vector3<f64>,
}