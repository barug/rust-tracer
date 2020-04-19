extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};
extern crate num_complex;

struct Coordinates {
    x: u32,
    y: u32
}

impl Coordinates {
    fn new(x: u32, y: u32) -> Coordinates {
        Coordinates{x: x, y: y}
    }
}

fn draw_rectangle<T>(img: &mut T, pixel: T::Pixel, dimensions: Coordinates, position: Coordinates)
    where T: GenericImage + GenericImageView
{
    for dx in 0..dimensions.x {
        for dy in 0..dimensions.y {
            img.put_pixel(position.x + dx, position.y + dy, pixel)
        }
    }
}

fn draw_circle<T>(img: &mut T, pixel: T::Pixel, r: u32, position: Coordinates)
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

fn draw_line<T>(img: &mut T, pixel: T::Pixel, p1: Coordinates, p2: Coordinates)
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

fn main() {
    let imgx = 800;
    let imgy = 800;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = RgbImage::new(imgx, imgy);

    imgbuf = draw_rectangle(imgbuf, Rgb([100, 100, 100]), 100, 50, 200, 200);
    imgbuf = draw_circle(imgbuf, Rgb([100, 100, 100]), 200, 300, 300);

    // Iterate over the coordinates and pixels of the image
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let r = (0.3 * x as f32) as u8;
    //     let b = (0.3 * y as f32) as u8;
    //     *pixel = image::Rgb([r, 0, b]);
    // }

    // A redundant loop to demonstrate reading image data
    // for x in 0..imgx {
    //     for y in 0..imgy {
    //         let cx = y as f32 * scalex - 1.5;
    //         let cy = x as f32 * scaley - 1.5;

    //         let c = num_complex::Complex::new(-0.4, 0.6);
    //         let mut z = num_complex::Complex::new(cx, cy);

    //         let mut i = 0;
    //         while i < 255 && z.norm() <= 2.0 {
    //             z = z * z + c;
    //             i += 1;
    //         }

    //         let pixel = imgbuf.get_pixel_mut(x, y);
    //         let image::Rgb(data) = *pixel;
    //         *pixel = image::Rgb([data[0], i as u8, data[2]]);
    //     }
    // }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}

