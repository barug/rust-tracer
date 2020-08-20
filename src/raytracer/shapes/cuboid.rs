use image::Rgb;
use serde::{Serialize, Deserialize};

use std::cmp::max;
use std::cmp::min;


use super::shape::*; 
use crate::coordinates::Coordinates3D;
use crate::raytracer::line::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cuboid {
    pub position: Coordinates3D,
    pub bounds: [Coordinates3D; 2],
    // pub min_bound: Coordinates3D,
    // pub max_bound: Coordinates3D,
    pub color: [u8; 3]
}

impl Cuboid {
    pub fn new(position: Coordinates3D, bounds: [Coordinates3D; 2], color: [u8; 3]) -> Cuboid {
        Cuboid {
            position: position,
            bounds : bounds,
            color: color
        }
    }
}

#[typetag::serde]
impl Shape3D for Cuboid {
    fn ray_closest_intersections (&self, ray: &Line) -> Option<(Coordinates3D, f64)> {

        let translated_origin: Coordinates3D = &ray.origin - &self.position;

        // let inverse_unit = 1.0 / &ray.unit_vec;
        // let sign: [usize; 3] = [
        //     (inverse_unit.x < 0.0) as usize,
        //     (inverse_unit.y < 0.0) as usize,
        //     (inverse_unit.z < 0.0) as usize,
        // ];

        // let txmin: f64 = (self.bounds[sign[0]].x - ray.origin.x) * inverse_unit.x;
        // let txmax: f64 = (self.bounds[1-sign[0]].x - ray.origin.x) * inverse_unit.x; 
        // let tymin: f64 = (self.bounds[sign[1]].y - ray.origin.y) * inverse_unit.y; 
        // let tymax: f64 = (self.bounds[1-sign[1]].y - ray.origin.y) * inverse_unit.y; 
    
        // if (txmin > tymax) || (tymin > txmax) {
        //     return None; 
        // }

        // let txymin: f64 = txmin.max(tymin);
        // let txymax: f64 = txmax.min(tymax);

        // // if tymin > txmin {
        // //     txmin = tymin;
        // // }
        // // if tymax < txmax {
        // //     txmax = tymax;
        // // }
        // let tzmin: f64 = (self.bounds[sign[2]].z - ray.origin.z) * inverse_unit.z; 
        // let tzmax: f64 = (self.bounds[1-sign[2]].z - ray.origin.z) * inverse_unit.z; 

        // if (txymin > tzmax) || (tzmin > txymax) {
        //     return None;
        // }

        // // if (tzmin > tmin) {
        // //     tmin = tzmin;
        // // }
        // // if (tzmax < tmax) {
        // //     tmax = tzmax;
        // // }
        // let txyzmin: f64 = txymin.max(tzmin);
        // let txyzmax: f64 = txymax.min(tzmax);

        
        // // println!("{} {}", txyzmin, txyzmax);
        // // let dist = if txyzmin < 0.0 {txyzmax} else {txyzmin};
        // let dist = txyzmin;
        // if txyzmin > txyzmax {
        //     return None
        // }
        // if dist < 0.0 {
        //     return None
        // }


        // // println!("unit : {:?}, inverse: {:?}, dist: {}", ray.unit_vec, inverse_unit, dist);
        //     // println!("inverse: {:?}", inverse_unit);
        //     // println!("{} ", dist);

        // let intersection = &ray.origin + &ray.unit_vec * dist;
        // // let intersection: Coordinates3D = Coordinates3D::new(txmin, tymin, tzmin);
        // // let dist: f64 = ray.origin.dist(&intersection);
        // return Some((intersection, dist));

        let txmin: f64 = (self.bounds[0].x - ray.origin.x) / ray.unit_vec.x;
        let txmax: f64 = (self.bounds[1].x - ray.origin.x) / ray.unit_vec.x; 
        let tymin: f64 = (self.bounds[0].y - ray.origin.y) / ray.unit_vec.y; 
        let tymax: f64 = (self.bounds[1].y - ray.origin.y) / ray.unit_vec.y; 
    
        let tzmin: f64 = (self.bounds[0].z - ray.origin.z) / ray.unit_vec.z; 
        let tzmax: f64 = (self.bounds[1].z - ray.origin.z) / ray.unit_vec.z; 

        //let tmin: f64 = max(max(min(txmin, txmax), min(tymin, t4)), min(t5, t6));
        // let tmax: f64 = min(min(max(t1, t2), max(t3, t4)), max(t5, t6));
        let tmin: f64 = ((txmin.min(txmax)).max(tymin.min(tymax))).max(tzmin.min(tzmax));
        let tmax: f64 = ((txmax.max(txmin)).min(tymax.max(tymin))).min(tzmax.max(tzmin));

            // let txymax: f64 = txmax.max.(txmin).min(tymax.max.(tymin))

        
        // println!("{} {}", txyzmin, txyzmax);
        if tmin > tmax || tmin < 0.0 {
            return None
        }


        // println!("unit : {:?}, inverse: {:?}, dist: {}", ray.unit_vec, inverse_unit, dist);
        //     // println!("inverse: {:?}", inverse_unit);
            // println!("{} ", dist);

        let intersection = &ray.origin + &ray.unit_vec * tmin;
        // let intersection: Coordinates3D = Coordinates3D::new(txmin, tymin, tzmin);
        // let dist: f64 = ray.origin.dist(&intersection);
        return Some((intersection, tmin));
    }

    fn get_color (&self) -> [u8; 3] {
        return self.color;
    }
}

//     let inverse_unit = 1.0 / &ray.unit_vec;
    //     let sign: [usize; 3] = [
    //         (inverse_unit.x < 0.0) as usize,
    //         (inverse_unit.y < 0.0) as usize,
    //         (inverse_unit.z < 0.0) as usize,
    //     ];

    //     let txmin: f64 = (self.bounds[sign[0]].x - ray.origin.x) * inverse_unit.x;
    //     let txmax: f64 = (self.bounds[1-sign[0]].x - ray.origin.x) * inverse_unit.x; 
    //     let tymin: f64 = (self.bounds[sign[1]].y - ray.origin.y) * inverse_unit.y; 
    //     let tymax: f64 = (self.bounds[1-sign[1]].y - ray.origin.y) * inverse_unit.y; 
    
    //     if (txmin > tymax) || (tymin > txmax) {
    //         return None; 
    //     }

    //     let txymin: f64 = txmin.min(tymin);
    //     let txymax: f64 = txmax.max(tymax);

    //     // if tymin > txmin {
    //     //     txmin = tymin;
    //     // }
    //     // if tymax < txmax {
    //     //     txmax = tymax;
    //     // }
    //     let tzmin: f64 = (self.bounds[sign[2]].z - ray.origin.z) * inverse_unit.z; 
    //     let tzmax: f64 = (self.bounds[1-sign[2]].z - ray.origin.z) * inverse_unit.z; 

    //     if (txymin > tzmax) || (tzmin > txymax) {
    //         return None;
    //     }

    //     // if (tzmin > tmin) {
    //     //     tmin = tzmin;
    //     // }
    //     // if (tzmax < tmax) {
    //     //     tmax = tzmax;
    //     // }
    //     let txyzmin: f64 = txymin.min(tzmin);
    //     let txyzmax: f64 = txymax.max(tzmax);

    //     if txyzmin > txmin || txyzmin > tymin || txyzmin > tzmin {
    //         println!("error txyzmin")
    //     }
    //     if txyzmax < txmax || txyzmax < tymax || txyzmax < tzmax {
    //         println!("error txyzmin")
    //     }
        
    //     // println!("{} {}", txyzmin, txyzmax);
    //     let dist = if txyzmin < 0.0 {txyzmax} else {txyzmin};
    //     if dist < 0.0 {
    //         return None
    //     }


    //     println!("unit : {:?}, inverse: {:?}, dist: {}", ray.unit_vec, inverse_unit, dist);
    //         // println!("inverse: {:?}", inverse_unit);
    //         // println!("{} ", dist);

    //     let intersection = &ray.origin + &ray.unit_vec * dist;
    //     // let intersection: Coordinates3D = Coordinates3D::new(txmin, tymin, tzmin);
    //     // let dist: f64 = ray.origin.dist(&intersection);
    //     return Some((intersection, dist));
    // }

    // fn get_color (&self) -> [u8; 3] {
    //     return self.color;
    // }
