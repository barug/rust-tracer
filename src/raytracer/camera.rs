use serde::{Serialize, Deserialize};

use crate::coordinates::*;

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub cam_orient: Coordinates3D,
    pub cam_pos: Coordinates3D,
    pub up_vec: Coordinates3D,
}