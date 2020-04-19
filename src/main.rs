extern crate image;
use image::{RgbImage, Rgb, GenericImage, GenericImageView, Pixel};
extern crate num_complex;

mod drawing_2d;
mod coordinates;
use crate::coordinates::Coordinates;

fn main() {
    let imgx = 800;
    let imgy = 800;

    // Rgb.from_slice([100, 100, 100],);
    // // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = RgbImage::new(imgx, imgy);
    // let test: Coordinates = Coordinates([10, 20])

    drawing_2d::draw_grid(&mut imgbuf, Rgb([100, 100, 100]), 50);

    drawing_2d::draw_line(&mut imgbuf, Rgb([100, 100, 100]), Coordinates::new(200, 100), Coordinates::new(50, 0));
    drawing_2d::draw_line(&mut imgbuf, Rgb([100, 100, 100]), Coordinates::new(50, 0), Coordinates::new(200, 100));

    drawing_2d::draw_line(&mut imgbuf, Rgb([100, 100, 100]), Coordinates::new(300, 50), Coordinates::new(100, 250));
    drawing_2d::draw_line(&mut imgbuf, Rgb([100, 100, 100]), Coordinates::new(100, 250), Coordinates::new(300, 50));


    drawing_2d::draw_rectangle(&mut imgbuf, Rgb([100, 100, 100]), Coordinates::new(100, 50), Coordinates::new(200, 200));
    drawing_2d::draw_circle(&mut imgbuf, Rgb([100, 100, 100]), 200, Coordinates::new(300, 300));



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

