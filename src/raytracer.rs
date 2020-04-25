extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};
use crate::coordinates::Coordinates3D;

pub struct Line {
    pub origin: Coordinates3D,
    pub unit_vec: Coordinates3D
}

impl Sphere {

    // pub fn new(x :f64, y: f64, z: f64, r: f64) -> Sphere {
    //     Sphere {
    //         centre : Coordinates3D::new(x, y, z),
    //         r: r
    //     }
    // }

    pub fn new(centre: Coordinates3D, r: f64) -> Sphere {
        Sphere {
            centre : centre,
            r: r
        }
    }

    pub fn intersect_sphere(&self, line: &Line) -> Option<Vec<(Coordinates3D, f64)>> {

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
            let dist2: f64 = - line.unit_vec.dot(&or_sub_centr) + discriminant.sqrt();
            let intersect2: Coordinates3D = &line.origin + &line.unit_vec * dist2;
            return Some(vec![(intersect1, dist1), (intersect2, dist2)])
        }
    } 
}

pub struct Sphere {
    pub centre: Coordinates3D,
    pub r: f64
}

// Calculate unit vector of line form 2 points
pub fn calcul_line_vec(origin: &Coordinates3D, point: &Coordinates3D) -> Coordinates3D {
    let d: f64 = origin.dist(point);
    Coordinates3D::new(
        (point.x - origin.x) / d,
        (point.y - origin.y) / d,
        (point.z - origin.z) / d,
    )
}

pub fn raytracing<T>(img: &mut T)
    where T: GenericImage + GenericImageView
{
    let (dimx, dimy): (u32, u32) = img.dimensions();
    let num_pix: u32             = dimx * dimy; 
    
    let cam_orient: Coordinates3D = Coordinates3D::new(0.0, 0.0, 1.0); 
    let cam_pos: Coordinates3D    = Coordinates3D::new(0.0 ,0.0 ,0.0);
    let up_vec: Coordinates3D     = Coordinates3D::new(0.0 ,1.0 ,0.0);
    
    let dis_viewport: f64        = 1.0;
    let viewport_width: f64      = 1.0;


    for i in 0..num_pix {
        let pi_x: u32 = i % dimx;
        let pi_y: u32 = i / dimx;
        let pix_x_coord: f64 = 0.0 - viewport_width / 2.0 + (1.0 / dimx as f64) * (0.5 + (i as f64 / dimx as f64));
        let pix_y_coord: f64 = 0.0 - viewport_width / 2.0 + (1.0 / dimy as f64) * (0.5 + (i as f64 / dimy as f64));
        // println!("pi_x: {}, pi_y: {}, pix_x_coord: {}, pix_y_coord: {}", pi_x, pi_y, pix_x_coord, pix_y_coord);
        let pos_pix: Coordinates3D = Coordinates3D::new(pix_x_coord, pix_y_coord, dis_viewport);

    }
}