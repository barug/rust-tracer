use na::Vector3;

use super::shape::Shape3D;

#[derive(Clone, Debug)]
pub struct Intersection<'a> {
    pub location: Vector3<f64>,
    pub biased_location: Vector3<f64>,
    pub distance: f64,
    pub normal: Vector3<f64>,
    pub shape: &'a dyn Shape3D
    // pub color: [u8; 3]
}

impl<'a> Intersection<'a> {
    pub fn new(location: Vector3<f64>, distance: f64, normal: Vector3<f64>, shape: &'a dyn Shape3D) -> Intersection<'a> {
        Intersection {
            location,
            // we use this biased_location to account for error margin in location and avoid ray colision with shape's surface
            biased_location: &location + 0.001 * &normal, 
            distance,
            normal,shape
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raytracer::shapes::Sphere;

    const EPSILON: f64 = 1e-10;
    const BIAS: f64 = 0.001;

    fn create_test_shape() -> Sphere {
        Sphere::new(
            Vector3::new(0.0, 0.0, 0.0),
            1.0,
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            0.5,
            0.0,
        )
    }

    #[test]
    fn test_intersection_stores_location() {
        let shape = create_test_shape();
        let location = Vector3::new(1.0, 2.0, 3.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        
        let intersection = Intersection::new(location, 5.0, normal, &shape);
        
        assert_eq!(intersection.location, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_intersection_stores_distance() {
        let shape = create_test_shape();
        let location = Vector3::new(1.0, 2.0, 3.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        
        let intersection = Intersection::new(location, 5.0, normal, &shape);
        
        assert!((intersection.distance - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_intersection_stores_normal() {
        let shape = create_test_shape();
        let location = Vector3::new(1.0, 2.0, 3.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        
        let intersection = Intersection::new(location, 5.0, normal, &shape);
        
        assert_eq!(intersection.normal, Vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_biased_location_offset_along_normal() {
        let shape = create_test_shape();
        let location = Vector3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);  // pointing up
        
        let intersection = Intersection::new(location, 1.0, normal, &shape);
        
        // Biased location should be offset along normal by 0.001
        assert!(intersection.biased_location.x.abs() < EPSILON);
        assert!((intersection.biased_location.y - BIAS).abs() < EPSILON);
        assert!(intersection.biased_location.z.abs() < EPSILON);
    }

    #[test]
    fn test_biased_location_with_arbitrary_normal() {
        let shape = create_test_shape();
        let location = Vector3::new(5.0, 5.0, 5.0);
        let normal = Vector3::new(1.0, 0.0, 0.0);  // pointing along X
        
        let intersection = Intersection::new(location, 1.0, normal, &shape);
        
        assert!((intersection.biased_location.x - (5.0 + BIAS)).abs() < EPSILON);
        assert!((intersection.biased_location.y - 5.0).abs() < EPSILON);
        assert!((intersection.biased_location.z - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_biased_location_negative_normal() {
        let shape = create_test_shape();
        let location = Vector3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, -1.0, 0.0);  // pointing down
        
        let intersection = Intersection::new(location, 1.0, normal, &shape);
        
        // Biased location should be offset in negative Y
        assert!((intersection.biased_location.y - (-BIAS)).abs() < EPSILON);
    }

    #[test]
    fn test_intersection_references_shape() {
        let shape = create_test_shape();
        let location = Vector3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        
        let intersection = Intersection::new(location, 1.0, normal, &shape);
        
        // Verify shape properties are accessible
        assert_eq!(intersection.shape.get_color(), Vector3::new(1.0, 0.0, 0.0));
        assert!((intersection.shape.get_albedo() - 0.5).abs() < EPSILON);
    }
}