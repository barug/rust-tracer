use image::Pixel;

use super::shape::*; 
use crate::coordinates::*;
use crate::raytracer::line::*;


pub struct Sphere<P>
    where P: Pixel
{
    pub centre: Coordinates3D,
    pub r: f64,
    pub color: P
}

impl<P> Sphere<P> 
    where P: Pixel
{

    // pub fn new(x :f64, y: f64, z: f64, r: f64) -> Sphere {
    //     Sphere {
    //         centre : Coordinates3D::new(x, y, z),
    //         r: r
    //     }
    // }

    pub fn new(centre: Coordinates3D, r: f64, color: P) -> Sphere<P> {
        Sphere {
            centre : centre,
            r: r,
            color: color
        }
    }
}

impl<P> Shape3D<P> for Sphere<P>
    where P: Pixel
{

    fn line_intersections (&self, line: &Line) -> Option<Vec<(Coordinates3D, f64)>> {
        let or_sub_centr = &line.origin - &self.centre;
        let discriminant: f64 = line.unit_vec.dot(&or_sub_centr).powi(2) - (or_sub_centr.norm().powi(2) - self.r.powi(2));

        if discriminant < 0.0 {
            return None
        }
        else if discriminant == 0.0 {
            let dist: f64 = - line.unit_vec.dot(&or_sub_centr);
            let intersect: Coordinates3D = &line.origin + &line.unit_vec * dist;
            return Some(vec![(intersect, dist)])
        } else {
            let dist1: f64 = - line.unit_vec.dot(&or_sub_centr) + discriminant.sqrt();
            let intersect1: Coordinates3D = &line.origin + &line.unit_vec * dist1;

            let dist2: f64 = - line.unit_vec.dot(&or_sub_centr) - discriminant.sqrt();
            let intersect2: Coordinates3D = &line.origin + &line.unit_vec * dist2;
            return Some(vec![(intersect1, dist1), (intersect2, dist2)])
        }
    }

    // fn closest_line_interstection (&self, line: &Line) -> Option<(Coordinates3D, f64)> {
    //     let or_sub_centr = &line.origin - &self.centre;
    //     let discriminant: f64 = line.unit_vec.dot(&or_sub_centr).powi(2) - (or_sub_centr.norm().powi(2) - self.r.powi(2));

    //     if discriminant < 0.0 {
    //         return None
    //     } else {
    //         let dist: f64 = - line.unit_vec.dot(&or_sub_centr) - discriminant.sqrt();
    //         let intersect1: Coordinates3D = &line.origin + &line.unit_vec * dist;
    //         return Some((intersect1, dist1))
    //     }
    // }

    fn get_color (&self) -> P {
        return self.color;
    }
}
