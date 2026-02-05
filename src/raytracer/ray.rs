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
        let unit: Vector3<f64> = (other - origin).normalize();
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

    pub fn new_from_origine_and_direction(origin: &Vector3<f64>, direction: &Vector3<f64>) -> Ray {
        let unit: Vector3<f64> = direction.normalize();
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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_ray_from_points_unit_vector_is_normalized() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let other = Vector3::new(3.0, 4.0, 0.0);
        let ray = Ray::new_from_points(&origin, &other);
        
        assert!((ray.unit_vec.norm() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_ray_from_points_direction() {
        let origin = Vector3::new(1.0, 2.0, 3.0);
        let other = Vector3::new(4.0, 6.0, 3.0); // direction (3, 4, 0), normalized (0.6, 0.8, 0)
        let ray = Ray::new_from_points(&origin, &other);
        
        assert!((ray.unit_vec.x - 0.6).abs() < EPSILON);
        assert!((ray.unit_vec.y - 0.8).abs() < EPSILON);
        assert!(ray.unit_vec.z.abs() < EPSILON);
    }

    #[test]
    fn test_ray_from_points_preserves_origin() {
        let origin = Vector3::new(1.0, 2.0, 3.0);
        let other = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new_from_points(&origin, &other);
        
        assert_eq!(ray.origin, origin);
    }

    #[test]
    fn test_ray_from_direction_normalizes() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(10.0, 0.0, 0.0);
        let ray = Ray::new_from_origine_and_direction(&origin, &direction);
        
        assert!((ray.unit_vec.norm() - 1.0).abs() < EPSILON);
        assert!((ray.unit_vec.x - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_ray_inverse_calculation() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(1.0, 2.0, 4.0);
        let ray = Ray::new_from_origine_and_direction(&origin, &direction);
        
        // inverse should be 1/unit_vec for each component
        assert!((ray.inverse.x * ray.unit_vec.x - 1.0).abs() < EPSILON);
        assert!((ray.inverse.y * ray.unit_vec.y - 1.0).abs() < EPSILON);
        assert!((ray.inverse.z * ray.unit_vec.z - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_ray_sign_positive_direction() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(1.0, 1.0, 1.0);
        let ray = Ray::new_from_origine_and_direction(&origin, &direction);
        
        assert_eq!(ray.sign, [0, 0, 0]);
    }

    #[test]
    fn test_ray_sign_negative_direction() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(-1.0, -1.0, -1.0);
        let ray = Ray::new_from_origine_and_direction(&origin, &direction);
        
        assert_eq!(ray.sign, [1, 1, 1]);
    }

    #[test]
    fn test_ray_sign_mixed_direction() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(1.0, -1.0, 1.0);
        let ray = Ray::new_from_origine_and_direction(&origin, &direction);
        
        assert_eq!(ray.sign, [0, 1, 0]);
    }
}