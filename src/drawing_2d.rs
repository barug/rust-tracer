extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};
extern crate num_complex;

// mod coordinates;
use crate::coordinates::Coordinates;


pub fn draw_rectangle<T>(img: &mut T, pixel: T::Pixel, dimensions: Coordinates, position: Coordinates)
    where T: GenericImage + GenericImageView
{
    for dx in 0..dimensions.x {
        for dy in 0..dimensions.y {
            img.put_pixel(position.x + dx, position.y + dy, pixel)
        }
    }
}

pub fn draw_circle<T>(img: &mut T, pixel: T::Pixel, r: u32, position: Coordinates)
    where T: GenericImage + GenericImageView
{
    // this value is the minimum range limit I found to get a continuous circle :
    // x = r/2
    // -> y = sqrt(3) / 2 * r
    // it might not be optimal
    let range: u32 = ((3.0_f64.sqrt() / 2.0) * r as f64)  as u32;

    for i in 0..=range {
        let dx: u32 = i;
        let dy: u32 = ((r as f64).powi(2) - (dx as f64).powi(2)).sqrt() as u32;
        img.put_pixel(position.x + dx, position.y + dy, pixel);
        img.put_pixel(position.x + dx, position.y - dy, pixel);
        img.put_pixel(position.x - dx, position.y + dy, pixel);
        img.put_pixel(position.x - dx, position.y - dy, pixel);

        let dy: u32 = i;
        let dx: u32 = ((r as f64).powi(2) - (dy as f64).powi(2)).sqrt() as u32;
        img.put_pixel(position.x + dx, position.y + dy, pixel);
        img.put_pixel(position.x + dx, position.y - dy, pixel);
        img.put_pixel(position.x - dx, position.y + dy, pixel);
        img.put_pixel(position.x - dx, position.y - dy, pixel);
    }
}

pub fn draw_line<T>(img: &mut T, pixel: T::Pixel, p1: Coordinates, p2: Coordinates)
    where T: GenericImage + GenericImageView
{
    let pdistx: i32 = p2.x as i32 - p1.x as i32;
    let pdisty: i32 = p2.y as i32 - p1.y as i32;
    
    if pdistx.abs() > pdisty.abs() {
        let tanx: f64 = pdisty as f64 / pdistx as f64;
        for abs_dx in 0..=pdistx.abs() {
            let dx = abs_dx * pdistx.signum();
            let dy: i32 = (dx as f64 * tanx) as i32;
            img.put_pixel((p1.x as i32 + dx) as u32, (p1.y as i32 + dy) as u32, pixel);
        }
    }
    else {
        let tany: f64 = pdistx as f64 / pdisty as f64;
        for abs_dy in 0..=pdisty.abs() {
            let dy = abs_dy * pdisty.signum();
            let dx: i32 = (dy as f64 * tany) as i32;
            img.put_pixel((p1.x as i32 + dx) as u32, (p1.y as i32 + dy) as u32, pixel);
        }
    }
}

pub fn draw_grid<T>(img: &mut T, pixel: T::Pixel, spacing: usize)
    where T: GenericImage + GenericImageView
{
    let (dimx, dimy) = img.dimensions();

    for x in (0..dimx).step_by(spacing) {
        for y in 0..dimy {
            img.put_pixel(x, y, pixel);
        }
    }

    for y in (0..dimy).step_by(spacing) {
        for x in 0..dimx {
            img.put_pixel(x, y, pixel);
        }
    }
}
