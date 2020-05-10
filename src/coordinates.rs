use std::ops;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates2D {
    pub x: u32,
    pub y: u32
}

impl Coordinates2D {
    pub fn new(x: u32, y: u32) -> Coordinates2D {
        Coordinates2D{x: x, y: y}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Coordinates3D {
    pub fn new(x: f64, y: f64, z: f64) -> Coordinates3D {
        Coordinates3D{x: x, y: y, z: z}
    }

    pub fn dot(&self, other: &Coordinates3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn dist(&self, other: &Coordinates3D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl ops::Add for &Coordinates3D {
    type Output = Coordinates3D;
    
    fn add(self, _rhs: &Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self.x + _rhs.x,
            self.y + _rhs.y,
            self.z + _rhs.z
        )
    }
}

impl ops::Add<Coordinates3D> for &Coordinates3D {
    type Output = Coordinates3D;
    
    fn add(self, _rhs: Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self.x + _rhs.x,
            self.y + _rhs.y,
            self.z + _rhs.z
        )
    }
}

impl ops::Add<&Coordinates3D> for Coordinates3D {
    type Output = Coordinates3D;
    
    fn add(self, _rhs: &Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self.x + _rhs.x,
            self.y + _rhs.y,
            self.z + _rhs.z
        )
    }
}


impl ops::Add for Coordinates3D {
    type Output = Coordinates3D;
    
    fn add(self, _rhs: Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self.x + _rhs.x,
            self.y + _rhs.y,
            self.z + _rhs.z
        )
    }
}


impl ops::Sub for &Coordinates3D {
    type Output = Coordinates3D;
    
    fn sub(self, _rhs: &Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self.x - _rhs.x,
            self.y - _rhs.y,
            self.z - _rhs.z
        )
    }
}

impl ops::Sub<Coordinates3D> for &Coordinates3D {
    type Output = Coordinates3D;
    
    fn sub(self, _rhs: Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self.x - _rhs.x,
            self.y - _rhs.y,
            self.z - _rhs.z
        )
    }
}

impl ops::Sub<&Coordinates3D> for Coordinates3D {
    type Output = Coordinates3D;
    
    fn sub(self, _rhs: &Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self.x - _rhs.x,
            self.y - _rhs.y,
            self.z - _rhs.z
        )
    }
}

impl ops::Sub for Coordinates3D {
    type Output = Coordinates3D;
    
    fn sub(self, _rhs: Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self.x - _rhs.x,
            self.y - _rhs.y,
            self.z - _rhs.z
        )
    }
}


impl ops::Mul<f64> for &Coordinates3D {
    type Output = Coordinates3D;
    
    fn mul(self, _rhs: f64) -> Coordinates3D {
        Coordinates3D::new(
            self.x * _rhs,
            self.y * _rhs,
            self.z * _rhs
        )
    }
}

impl ops::Mul<f64> for Coordinates3D {
    type Output = Coordinates3D;
    
    fn mul(self, _rhs: f64) -> Coordinates3D {
        Coordinates3D::new(
            self.x * _rhs,
            self.y * _rhs,
            self.z * _rhs
        )
    }
}

impl ops::Mul<&Coordinates3D> for f64 {
    type Output = Coordinates3D;
    
    fn mul(self, _rhs: &Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self * _rhs.x,
            self * _rhs.y,
            self * _rhs.z
        )
    }
}

impl ops::Mul<Coordinates3D> for f64 {
    type Output = Coordinates3D;
    
    fn mul(self, _rhs: Coordinates3D) -> Coordinates3D {
        Coordinates3D::new(
            self * _rhs.x,
            self * _rhs.y,
            self * _rhs.z
        )
    }
}