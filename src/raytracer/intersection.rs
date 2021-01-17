use na::Vector3;


pub struct Intersection {
    pub location: Vector3<f64>,
    pub distance: f64,
    pub normal: Vector3<f64>,
    pub color: [u8; 3]
}

impl Intersection {
    pub fn new(location: Vector3<f64>, distance: f64, normal: Vector3<f64>, color: [u8; 3]) -> Intersection {
        Intersection{location: location, distance: distance, normal: normal, color: color}
    }
}