use serde::{Serialize, Deserialize};
use na::Vector3;

use super::shape::*; 
use crate::raytracer::ray::*;
use crate::raytracer::Intersection;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plane {
    pub origin: Vector3<f64>,
    pub normal_vec: Vector3<f64>,
    pub color: Vector3<f64>,
    pub emissive_color: Vector3<f64>,
    albedo: f64,
    pub reflectivity: f64
}

impl Plane {
    pub fn new(
        origin: Vector3<f64>, 
        normal_vec: Vector3<f64>, 
        color: Vector3<f64>, 
        emissive_color: Vector3<f64>, 
        albedo: f64, 
        reflectivity : f64
    ) -> Plane {
        Plane{ origin, normal_vec, color, emissive_color, albedo, reflectivity}
    }
}

#[typetag::serde]
impl Shape3D for Plane {
    fn ray_closest_intersections (&self, ray: &Ray) -> Option<Intersection> {
        let l_dot_n: f64 = ray.unit_vec.dot(&self.normal_vec);
        if l_dot_n != 0.0 {
            let distance = (&self.origin - &ray.origin).dot(&self.normal_vec) / l_dot_n;
            if distance < 0.0 {
                return None
            }
            let location = &ray.origin + &ray.unit_vec * distance;
            let normal: Vector3<f64> = if ray.unit_vec.angle(&self.normal_vec) > std::f64::consts::PI / 2.0 {
                self.normal_vec.clone_owned()
            } else {
                - self.normal_vec.clone_owned()
            };
            // println!("{:?}", &ray.unit_vec);
            // println!("{:?}", ray.unit_vec.angle(&self.normal_vec));
            // println!("{:?}", normal);
            return Some(Intersection::new(location, distance, normal, self))
        }
        None
    }

    fn get_color (&self) -> Vector3<f64> {
        return self.color;
    }

    fn get_emissive_color (&self) -> Vector3<f64> {
        return self.emissive_color;
    }

    fn get_albedo (&self) -> f64 {
        return self.albedo;
    }

    fn get_reflectivity(&self) -> f64 {
        return self.reflectivity;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn create_floor_plane() -> Plane {
        Plane::new(
            Vector3::new(0.0, 0.0, 0.0),   // origin at world origin
            Vector3::new(0.0, 1.0, 0.0),   // normal pointing up (Y+)
            Vector3::new(0.5, 0.5, 0.5),   // gray
            Vector3::new(0.0, 0.0, 0.0),   // no emission
            0.8,                            // albedo
            0.0,                            // no reflection
        )
    }

    #[test]
    fn test_plane_ray_hit_from_above() {
        let plane = create_floor_plane();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 5.0, 0.0),   // start above plane
            &Vector3::new(0.0, -1.0, 0.0),  // shoot downward
        );
        
        let intersection = plane.ray_closest_intersections(&ray);
        assert!(intersection.is_some());
        
        let hit = intersection.unwrap();
        assert!((hit.distance - 5.0).abs() < EPSILON);
        assert!(hit.location.y.abs() < EPSILON); // should hit at y=0
    }

    #[test]
    fn test_plane_ray_hit_from_below() {
        let plane = create_floor_plane();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, -3.0, 0.0),  // start below plane
            &Vector3::new(0.0, 1.0, 0.0),   // shoot upward
        );
        
        let intersection = plane.ray_closest_intersections(&ray);
        assert!(intersection.is_some());
        
        let hit = intersection.unwrap();
        assert!((hit.distance - 3.0).abs() < EPSILON);
    }

    #[test]
    fn test_plane_ray_parallel_miss() {
        let plane = create_floor_plane();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 5.0, 0.0),
            &Vector3::new(1.0, 0.0, 0.0),  // parallel to plane
        );
        
        let intersection = plane.ray_closest_intersections(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_plane_ray_opposite_direction() {
        let plane = create_floor_plane();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 5.0, 0.0),
            &Vector3::new(0.0, 1.0, 0.0),  // shooting away from plane
        );
        
        let intersection = plane.ray_closest_intersections(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_plane_normal_faces_ray_from_above() {
        let plane = create_floor_plane();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 5.0, 0.0),
            &Vector3::new(0.0, -1.0, 0.0),
        );
        
        let hit = plane.ray_closest_intersections(&ray).unwrap();
        // Normal should face the ray (point upward, Y+)
        assert!((hit.normal.y - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_plane_normal_faces_ray_from_below() {
        let plane = create_floor_plane();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, -5.0, 0.0),
            &Vector3::new(0.0, 1.0, 0.0),
        );
        
        let hit = plane.ray_closest_intersections(&ray).unwrap();
        // Normal should face the ray (point downward, Y-)
        assert!((hit.normal.y - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_plane_hit_at_angle() {
        let plane = create_floor_plane();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 5.0, -5.0),
            &Vector3::new(0.0, -1.0, 1.0).normalize(),  // diagonal
        );
        
        let intersection = plane.ray_closest_intersections(&ray);
        assert!(intersection.is_some());
        
        let hit = intersection.unwrap();
        assert!(hit.location.y.abs() < EPSILON);  // should hit at y=0
        assert!(hit.location.z.abs() < EPSILON);  // should hit at z=0
    }

    #[test]
    fn test_vertical_plane() {
        let wall = Plane::new(
            Vector3::new(5.0, 0.0, 0.0),    // wall at x=5
            Vector3::new(-1.0, 0.0, 0.0),  // normal pointing toward origin
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(0.0, 0.0, 0.0),
            0.5,
            0.0,
        );
        
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(1.0, 0.0, 0.0),  // shoot toward wall
        );
        
        let hit = wall.ray_closest_intersections(&ray).unwrap();
        assert!((hit.distance - 5.0).abs() < EPSILON);
        assert!((hit.location.x - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_plane_get_color() {
        let plane = create_floor_plane();
        let color = plane.get_color();
        assert_eq!(color, Vector3::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_plane_get_albedo() {
        let plane = create_floor_plane();
        assert!((plane.get_albedo() - 0.8).abs() < EPSILON);
    }
}