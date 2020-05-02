pub mod line;
pub mod shapes;
pub mod scene;

extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};

use crate::coordinates::Coordinates3D;
pub use self::shapes::*;
pub use self::line::*;
pub use self::scene::*;

pub fn raytracing<T>(img: &mut T, scene: Scene<T::Pixel>)
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
        let pix_x_coord: f64 = 0.0 - viewport_width / 2.0 + (1.0 / dimx as f64) * (0.5 + pi_x as f64);
        let pix_y_coord: f64 = 0.0 - viewport_width / 2.0 + (1.0 / dimy as f64) * (0.5 + pi_y as f64);
        
        // we have -pix_y_coord for pixel y position because y axis is reversed on screen
        let pos_pix: Coordinates3D = Coordinates3D::new(pix_x_coord, -pix_y_coord, dis_viewport);
        let ray: Line = Line::new_from_points(&cam_pos, &pos_pix);

        let mut closest: f64 = std::f64::INFINITY;

        for shape in &scene.shapes {
            let result = shape.ray_closest_intersections(&ray);
            if let Some(intersection) = &result {
                if intersection.1 < closest {
                    img.put_pixel(pi_x, pi_y, shape.get_color());
                    closest = intersection.1;
                }
            }
        }
    }
}