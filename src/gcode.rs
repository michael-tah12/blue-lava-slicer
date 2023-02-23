use nalgebra::Vector3;

use crate::read_config::Config;

pub fn create_from_paths(paths: &Vec<Vec<[Vector3<f64>; 2]>>, config: &Config) -> String {
    let mut gcode: String = String::from("");
    let mut start = Vector3::new(0.0, 0.0, 0.0);
    let mut end = Vector3::new(0.0, 0.0, 0.0);
    for slice in paths.iter() {
        let mut slice_retain = slice.to_owned();
        for i in 0..slice.len() {
            let (path, start_point_idx) = find_nearest_path(&slice_retain, &start);
            start = path[start_point_idx];
            let end_point_idx = if start_point_idx == 0 { 1 } else { 0 };
            end = path[end_point_idx];
            let path_vec = end - start;
            let length = path_vec.norm();
            let path_volume = config.quality.line_width * config.quality.layer_height * length;
            let radius: f64 = config.general.filament_diameter / 2.0;
            let filament_length = path_volume / (3.14 * radius.powf(2.0));
            let line = format!(
                "{} X{} Y{} E{:.2}\n",
                "G1", path_vec[0], path_vec[1], filament_length
            );
            gcode.push_str(line.as_str());

            slice_retain.retain(|p| *p != path);
            start = end;
        }
    }
    return gcode;
}

pub fn displace_paths(paths: &mut Vec<Vec<[Vector3<f64>; 2]>>, placement: &[f64; 3]) {
    for slice in paths.into_iter() {
        for slice_paths in slice.into_iter() {
            for path in slice_paths.into_iter() {
                path[0] += placement[0];
                path[1] += placement[1];
                path[2] += placement[2];
            }
        }
    }
}

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
