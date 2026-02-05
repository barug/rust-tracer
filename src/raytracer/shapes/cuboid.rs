use serde::{Serialize, Deserialize};
use na::Vector3;

use super::shape::*; 
use crate::raytracer::ray::*;
use crate::raytracer::Intersection;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cuboid {
    pub position: Vector3<f64>,
    pub bounds: [Vector3<f64>; 2], 
    pub color: Vector3<f64>,
    pub emissive_color: Vector3<f64>,
    pub albedo: f64,
    pub reflectivity: f64
}

impl Cuboid {
    pub fn new(
        position: Vector3<f64>, 
        bounds: [Vector3<f64>; 2], 
        color: Vector3<f64>,  
        emissive_color: Vector3<f64>, 
        albedo: f64,
        reflectivity: f64
    ) -> Cuboid {
        Cuboid {position, bounds, color, emissive_color, albedo, reflectivity}
    }
}

#[typetag::serde]
impl Shape3D for Cuboid {

    // Bouding box AABB algorithm such as seen at :
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection
    // saved in doc folder in case of dead link
    fn ray_closest_intersections (&self, ray: &Ray) -> Option<Intersection> {

        let translated_origin: Vector3<f64> = &ray.origin - &self.position;

        let txmin: f64 = (self.bounds[ray.sign[0]].x - translated_origin.x) * ray.inverse.x;
        let txmax: f64 = (self.bounds[1-ray.sign[0]].x - translated_origin.x) * ray.inverse.x; 
        let tymin: f64 = (self.bounds[ray.sign[1]].y - translated_origin.y) * ray.inverse.y; 
        let tymax: f64 = (self.bounds[1-ray.sign[1]].y - translated_origin.y) * ray.inverse.y; 
    
        if (txmin > tymax) || (tymin > txmax) {
            return None; 
        }

        let txymin: f64 = txmin.max(tymin);
        let txymax: f64 = txmax.min(tymax);

        let tzmin: f64 = (self.bounds[ray.sign[2]].z - translated_origin.z) * ray.inverse.z; 
        let tzmax: f64 = (self.bounds[1-ray.sign[2]].z - translated_origin.z) * ray.inverse.z; 

        if (txymin > tzmax) || (tzmin > txymax) {
            return None;
        }

        let txyzmin: f64 = txymin.max(tzmin);
        let txyzmax: f64 = txymax.min(tzmax);

 
        let distance = txyzmin;
        if txyzmin > txyzmax {
            return None
        }
        if distance < 0.0 {
            return None
        }

        let location = &ray.origin + &ray.unit_vec * distance;
        let normalized_location = &location - &self.position;

        // the normal is on the axis with the highest absolute value
        let mut normal: Vector3<f64> = Vector3::<f64>::zeros();
        let normal_abs: f64 = 1.0;
        normal[normalized_location.iamax()] = normal_abs.copysign(normalized_location[normalized_location.iamax()]);

        return Some(Intersection::new(location, distance, normal, self))
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

    fn create_test_cuboid() -> Cuboid {
        Cuboid::new(
            Vector3::new(0.0, 0.0, 5.0),  // position at z=5
            [
                Vector3::new(-1.0, -1.0, -1.0),  // min bounds
                Vector3::new(1.0, 1.0, 1.0),     // max bounds (2x2x2 cube)
            ],
            Vector3::new(1.0, 0.0, 1.0),  // magenta
            Vector3::new(0.0, 0.0, 0.0),  // no emission
            0.5,
            0.0,
        )
    }

    #[test]
    fn test_cuboid_ray_hit_front_face() {
        let cuboid = create_test_cuboid();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 0.0, 1.0),  // shoot along Z
        );
        
        let intersection = cuboid.ray_closest_intersections(&ray);
        assert!(intersection.is_some());
        
        let hit = intersection.unwrap();
        // Cuboid at z=5 with bounds -1 to 1, so front face is at z=4
        assert!((hit.distance - 4.0).abs() < EPSILON);
        assert!((hit.location.z - 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_cuboid_ray_hit_side_face() {
        let cuboid = create_test_cuboid();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(-5.0, 0.0, 5.0),  // to the left of cuboid
            &Vector3::new(1.0, 0.0, 0.0),   // shoot along X
        );
        
        let intersection = cuboid.ray_closest_intersections(&ray);
        assert!(intersection.is_some());
        
        let hit = intersection.unwrap();
        // Should hit left face at x=-1 (relative to position, so world x=-1)
        assert!((hit.distance - 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_cuboid_ray_miss() {
        let cuboid = create_test_cuboid();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(1.0, 0.0, 0.0),  // shoot along X, cuboid is at z=5
        );
        
        let intersection = cuboid.ray_closest_intersections(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_cuboid_ray_miss_opposite_direction() {
        let cuboid = create_test_cuboid();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 0.0, -1.0),  // shoot away from cuboid
        );
        
        let intersection = cuboid.ray_closest_intersections(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_cuboid_normal_front_face() {
        let cuboid = create_test_cuboid();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 0.0, 1.0),
        );
        
        let hit = cuboid.ray_closest_intersections(&ray).unwrap();
        // Normal should point toward camera (negative Z)
        assert!((hit.normal.z - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_cuboid_normal_top_face() {
        let cuboid = create_test_cuboid();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 5.0, 5.0),   // above the cuboid
            &Vector3::new(0.0, -1.0, 0.0),  // shoot downward
        );
        
        let hit = cuboid.ray_closest_intersections(&ray).unwrap();
        // Normal should point upward (positive Y)
        assert!((hit.normal.y - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_cuboid_diagonal_hit() {
        let cuboid = create_test_cuboid();
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(-5.0, 5.0, 0.0),
            &Vector3::new(1.0, -1.0, 1.0).normalize(),
        );
        
        let intersection = cuboid.ray_closest_intersections(&ray);
        assert!(intersection.is_some());
    }

    #[test]
    fn test_cuboid_ray_along_edge_miss() {
        let cuboid = create_test_cuboid();
        // Ray that passes just outside the cuboid
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(2.0, 0.0, 0.0),   // offset by more than bounds
            &Vector3::new(0.0, 0.0, 1.0),
        );
        
        let intersection = cuboid.ray_closest_intersections(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_cuboid_get_color() {
        let cuboid = create_test_cuboid();
        let color = cuboid.get_color();
        assert_eq!(color, Vector3::new(1.0, 0.0, 1.0));
    }

    #[test]
    fn test_cuboid_get_albedo() {
        let cuboid = create_test_cuboid();
        assert!((cuboid.get_albedo() - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_cuboid_get_reflectivity() {
        let cuboid = create_test_cuboid();
        assert!(cuboid.get_reflectivity().abs() < EPSILON);
    }

    #[test]
    fn test_cuboid_asymmetric_bounds() {
        let cuboid = Cuboid::new(
            Vector3::new(0.0, 0.0, 0.0),
            [
                Vector3::new(-1.0, -2.0, -3.0),
                Vector3::new(1.0, 2.0, 3.0),
            ],
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(0.0, 0.0, 0.0),
            0.5,
            0.0,
        );
        
        let ray = Ray::new_from_origine_and_direction(
            &Vector3::new(0.0, 0.0, -10.0),
            &Vector3::new(0.0, 0.0, 1.0),
        );
        
        let hit = cuboid.ray_closest_intersections(&ray).unwrap();
        assert!((hit.location.z - (-3.0)).abs() < EPSILON);
    }
}
