mod gcode;
mod inner_walls;
mod read_config;
mod read_stl;
mod slice;

use nalgebra::Vector3;
use std::env;
use std::fs;

use crate::inner_walls::create_inner_paths_from_outer;
use crate::read_config::read_config_toml;
use crate::read_stl::{read_stl_ascii, Triangle};
use crate::slice::trivial;

fn main() {
    let args: Vec<String> = env::args().collect();
    let stl_file: &str = &args[1];
    let gcode_file: String = stl_file.split(".stl").next().unwrap().to_owned() + ".gcode";
    let config_file: &str = &args[2];

    let config = read_config_toml(config_file);

    let triangles: Vec<Triangle> = read_stl_ascii::read(stl_file);
    let mut outer_paths: Vec<Vec<[Vector3<f64>; 2]>> = Vec::new();

    let mut slice_z = 0.0;
    let mut slice: Vec<[Vector3<f64>; 2]>;
    let mut keep_slicing = true;

    // slice outer paths
    while keep_slicing {
        slice = trivial::slice(&triangles, slice_z);
        if slice.len() == 0 {
            keep_slicing = false;
        } else {
            outer_paths.push(slice);
            slice_z += config.quality.layer_height;
        }
        keep_slicing = false
    }

    // inner paths from outer paths
    let inner_paths = create_inner_paths_from_outer(&outer_paths);

    let gcode_head = String::from(fs::read_to_string("header.gcode").expect(""));
    let gcode_str = gcode::create_from_paths(&outer_paths, &config);
    let gcode_complete = format!("{}{}", gcode_head, gcode_str);

    fs::write(gcode_file, &gcode_complete).expect("");
    //println!("{:?}", gcode_complete);
}
