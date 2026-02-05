use serde::{Serialize, Deserialize};

use super::shape::*; 
use crate::raytracer::ray::*;
use crate::raytracer::Intersection;

use na::Vector3;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub centre: Vector3<f64>,
    pub r: f64,
    pub color: Vector3<f64>,
    pub emissive_color: Vector3<f64>,
    pub albedo: f64,
    pub reflectivity: f64
}

impl Sphere {
    pub fn new(
        centre: Vector3<f64>, 
        r: f64, 
        color: Vector3<f64>, 
        emissive_color: Vector3<f64>, 
        albedo: f64,
        reflectivity: f64
    ) -> Sphere {
        Sphere {centre, r, color, emissive_color, albedo, reflectivity}
    }
}

#[typetag::serde]
impl Shape3D for Sphere {

    fn ray_closest_intersections (&self, ray: &Ray) -> Option<Intersection> {
        let or_sub_centr = &ray.origin - &self.centre;
        let discriminant: f64 = ray.unit_vec.dot(&or_sub_centr).powi(2) - (or_sub_centr.norm().powi(2) - self.r.powi(2));

        if discriminant < 0.0 {
            return None
        }
        else if discriminant == 0.0 {
            let dist: f64 = - ray.unit_vec.dot(&or_sub_centr);
            if dist < 0.0 { 
                return None
            }
        } else {
            let dist1: f64 = - ray.unit_vec.dot(&or_sub_centr) + discriminant.sqrt();            
            let dist2: f64 = - ray.unit_vec.dot(&or_sub_centr) - discriminant.sqrt();

            if dist1 >= 0.0 && (dist2 < 0.0 || dist2 > dist1) {
                let location: Vector3<f64> = &ray.origin + &ray.unit_vec * dist1;
                let normal: Vector3<f64> = (&location - &self.centre).normalize(); 
                return Some(Intersection::new(location, dist1, normal, self))
            } else if dist2 >= 0.0 {
                let location: Vector3<f64> = &ray.origin + &ray.unit_vec * dist2;
                let normal: Vector3<f64> = (&location - &self.centre).normalize();
                return Some(Intersection::new(location, dist2, normal, self))
            }
        }
        return None;
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

    fn create_test_sphere() -> Sphere {
        Sphere::new(
            Vector3::new(0.0, 0.0, 5.0),  // center at z=5
            1.0,                           // radius 1
            Vector3::new(1.0, 0.0, 0.0),  // red
            Vector3::new(0.0, 0.0, 0.0),  // no emission
            0.5,                           // albedo
            0.0,                           // no reflection
        )
    }

    #[test]
    fn test_sphere_ray_hit_front() {
        let sphere = create_test_sphere();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 0.0, 1.0),
        );
        
        let intersection = sphere.ray_closest_intersections(&ray);
        assert!(intersection.is_some());
        
        let hit = intersection.unwrap();
        assert!((hit.distance - 4.0).abs() < EPSILON); // sphere at z=5, radius 1, hit at z=4
        assert!((hit.location.z - 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_sphere_ray_miss() {
        let sphere = create_test_sphere();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(1.0, 0.0, 0.0),  // shooting along X, sphere is along Z
        );
        
        let intersection = sphere.ray_closest_intersections(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_sphere_ray_miss_opposite_direction() {
        let sphere = create_test_sphere();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 0.0, -1.0),  // shooting away from sphere
        );
        
        let intersection = sphere.ray_closest_intersections(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_sphere_ray_from_inside() {
        let sphere = Sphere::new(
            Vector3::new(0.0, 0.0, 0.0),  // center at origin
            2.0,                           // radius 2
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            0.5,
            0.0,
        );
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),  // origin at sphere center
            &Vector3::new(0.0, 0.0, 1.0),
        );
        
        let intersection = sphere.ray_closest_intersections(&ray);
        assert!(intersection.is_some());
        
        let hit = intersection.unwrap();
        assert!((hit.distance - 2.0).abs() < EPSILON); // should hit at radius
    }

    #[test]
    fn test_sphere_normal_points_outward() {
        let sphere = create_test_sphere();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 0.0, 1.0),
        );
        
        let hit = sphere.ray_closest_intersections(&ray).unwrap();
        // Normal should point toward camera (negative Z direction)
        assert!((hit.normal.z - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_sphere_tangent_ray() {
        // Ray that just grazes the sphere
        let sphere = create_test_sphere(); // center (0,0,5), radius 1
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(1.0, 0.0, 0.0),  // offset by radius
            &Vector3::new(0.0, 0.0, 1.0),
        );
        
        let intersection = sphere.ray_closest_intersections(&ray);
        // Tangent ray should still hit (discriminant = 0)
        assert!(intersection.is_some());
    }

    #[test]
    fn test_sphere_get_color() {
        let sphere = create_test_sphere();
        let color = sphere.get_color();
        assert_eq!(color, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_sphere_get_albedo() {
        let sphere = create_test_sphere();
        assert!((sphere.get_albedo() - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_sphere_get_reflectivity() {
        let sphere = create_test_sphere();
        assert!(sphere.get_reflectivity().abs() < EPSILON);
    }

    #[test]
    fn test_sphere_get_emissive_color() {
        let sphere = create_test_sphere();
        let emissive = sphere.get_emissive_color();
        assert_eq!(emissive, Vector3::new(0.0, 0.0, 0.0));
    }
}
