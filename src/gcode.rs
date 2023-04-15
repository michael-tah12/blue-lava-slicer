use nalgebra::Vector3;

use crate::read_config::Config;

pub fn create_from_paths(paths: &Vec<Vec<[Vector3<f64>; 2]>>, config: &Config) -> String {
    let mut gcode: String = String::from("");
    let mut start = Vector3::new(0.0, 0.0, 0.0);
    let mut end: Vector3<f64>;
    let mut found_path: bool;
    let mut path: [Vector3<f64>; 2];
    let mut start_point_idx: usize;
    let mut z_height: f64 = 0.0;
    let mut extruder_postion: f64 = 0.0;

    for (i, slice) in paths.iter().enumerate() {
        let mut slice_retain = slice.to_owned();
        for j in 0..slice.len() {
            if j == 0 {
                (path, start_point_idx) = find_nearest_path(&slice_retain, &start);
                found_path = true;
            } else {
                (found_path, path, start_point_idx) = find_connecting_path(&slice_retain, &start);
            }
            let line: String;
            if found_path {
                let end_point_idx = if start_point_idx == 0 { 1 } else { 0 };
                end = path[end_point_idx];
                line = create_line(&end, &mut extruder_postion, &config);
                slice_retain.retain(|p| *p != path);
                gcode.push_str(line.as_str());
                start = end;
            } else {
                (path, start_point_idx) = find_nearest_path(&slice_retain, &start);
                start = path[start_point_idx];
                let end_point_idx = if start_point_idx == 0 { 1 } else { 0 };
                end = path[end_point_idx];
                line = create_travel_move(&end);
                gcode.push_str(line.as_str());
            }
        }
        z_height += config.quality.layer_height;
        let z_hop = format!("{} Z{}\n", "G0", z_height);
        let layer_number = format!(";LAYER:{}\n", i);
        gcode.push_str(layer_number.as_str());
        gcode.push_str(z_hop.as_str());
    }
    return gcode;
}

// pub fn displace_paths(paths: &mut Vec<Vec<[Vector3<f64>; 2]>>, placement: &[f64; 3]) {
//     for slice in paths.into_iter() {
//         for slice_paths in slice.into_iter() {
//             for path in slice_paths.into_iter() {
//                 path[0] += placement[0];
//                 path[1] += placement[1];
//                 path[2] += placement[2];
//             }
//         }
//     }
// }

fn find_nearest_path(
    paths: &Vec<[Vector3<f64>; 2]>,
    point: &Vector3<f64>,
) -> ([Vector3<f64>; 2], usize) {
    let mut nearest_path = [Vector3::zeros(); 2];
    let mut nearest_point_idx: usize = 0;
    let mut parr_norm_min = 1000.0;
    let mut parr_norm: f64;
    for path in paths.iter() {
        for i in 0..2 {
            parr_norm = (path[i] - point).norm();
            if parr_norm < parr_norm_min {
                parr_norm_min = parr_norm;
                nearest_path = *path;
                nearest_point_idx = i;
            }
        }
    }
    return (nearest_path, nearest_point_idx);
}

fn find_connecting_path(
    paths: &Vec<[Vector3<f64>; 2]>,
    point: &Vector3<f64>,
) -> (bool, [Vector3<f64>; 2], usize) {
    let mut nearest_path = [Vector3::zeros(); 2];
    let mut nearest_point_idx: usize = 0;
    let mut parr_norm: f64;
    for path in paths.iter() {
        for i in 0..2 {
            parr_norm = (path[i] - point).norm();
            if parr_norm == 0.0 {
                nearest_path = *path;
                nearest_point_idx = i;
                return (true, nearest_path, nearest_point_idx);
            }
        }
    }
    return (false, nearest_path, nearest_point_idx);
}

fn create_line(end: &Vector3<f64>, extruder_position: &mut f64, config: &Config) -> String {
    let path_vec = end;
    let length = path_vec.norm();
    let path_volume = config.quality.line_width * config.quality.layer_height * length;
    let radius: f64 = config.general.filament_diameter / 2.0;
    let filament_length = path_volume / (3.14 * radius.powf(2.0));
    *extruder_position += filament_length;

    return format!(
        "{} X{} Y{} E{:.2}\n",
        "G1", path_vec[0], path_vec[1], *extruder_position
    );
}

fn create_travel_move(end: &Vector3<f64>) -> String {
    return format!("{} X{} Y{}\n", "G0", end[0], end[1]);
}
