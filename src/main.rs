mod gcode;
mod read_config;
mod read_stl;
mod slice;

use nalgebra::Vector3;
use std::fs;

use crate::read_config::read_config_toml;
use crate::read_stl::{read_stl_ascii, Triangle};
use crate::slice::trivial;

fn main() {
    let config = read_config_toml("config.toml");

    let triangles: Vec<Triangle> = read_stl_ascii::read("suzanne.stl");
    let mut outer_paths: Vec<Vec<[Vector3<f64>; 2]>> = Vec::new();

    let mut slice_z = 0.0;
    let mut slice: Vec<[Vector3<f64>; 2]>;
    let mut keep_slicing = true;
    while keep_slicing {
        slice = trivial::slice(&triangles, slice_z);
        if slice.len() == 0 {
            keep_slicing = false;
        } else {
            outer_paths.push(slice);
            slice_z += config.quality.layer_height;
        }
    }

    //gcode::displace_paths(&mut outer_paths, &config.general.placement);
    let gcode_head = String::from(fs::read_to_string("header.gcode").expect(""));
    let gcode_str = gcode::create_from_paths(&outer_paths, &config);
    let gcode_complete = format!("{}{}", gcode_head, gcode_str);

    fs::write("suzanne.gcode", &gcode_complete).expect("");
    println!("{:?}", gcode_complete);
}
