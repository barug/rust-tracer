extern crate image;
use image::{ImageBuffer, Rgb};
extern crate nalgebra as na;
extern crate num_complex;
extern crate clap;
use clap::{App, Arg};

mod coordinates;
mod drawing_2d;
mod raytracer;
use crate::raytracer::*;

use std::fs;


fn main() -> std::io::Result<()> {
    let matches = App::new("rust-tracer")
        .version("0.1")
        .author("Barthélémy Gouby")
        .about("Simple raytracer written in rust")
        .arg(
            Arg::with_name("SCENE")
                .help("The scene configuration file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("the output path")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("dimensions")
                .short("d")
                .long("dimensions")
                .value_name("WIDTHxHEIGHT")
                .help("the dimension of the output image")
                .takes_value(true)
        )
        .get_matches();


    let scene_path = matches.value_of("SCENE").unwrap();
    let output_path = matches.value_of("OUTPUT").unwrap();
    let dimensions_str = matches.value_of("dimensions").unwrap_or("640x480");


    let conf = fs::read_to_string(scene_path)?;
    let scene: Scene = serde_yaml::from_str(&conf).unwrap();

    let dimensions = dimensions_str.split('x').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let raw_pixels = scene.render_scene(dimensions[0], dimensions[1]);
    let imgbuf = ImageBuffer::<Rgb<u16>, Vec<u16>>::from_vec(dimensions[0], dimensions[1], raw_pixels).unwrap();

    // the format is deduced from the file extension in the output_path
    imgbuf.save(output_path).unwrap();
    Ok(())
}
