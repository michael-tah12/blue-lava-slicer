use std::f64::consts::PI;

use nalgebra::Vector3;

use crate::slice::Path;

pub fn create_inner_paths_from_outer(outer_paths: &Vec<Vec<Path>>) {
    let mut inner_paths: Vec<Vec<Path>> = Vec::new();
    for slice in outer_paths.iter() {
        let rep_slice = slice.repeat(2);
        let mut inner_path_slice: Vec<Path> = Vec::new();
        for (i, path) in rep_slice.iter().enumerate() {
            let next_path = rep_slice[i + 1];

            let angle = calculate_angle(&path, &next_path);

            let point0 = path.points[0] - path.normal;
            let point1 = path.points[1] - path.normal;
            let new_inner_path = Path {
                points: [point0, point1],
                normal: path.normal,
            };
            inner_path_slice.push(new_inner_path);
            println!("{:?}", new_inner_path);
            if i > outer_paths.len() + 1 {
                break;
            }
        }
        inner_paths.push(inner_path_slice)
    }
}

fn calculate_angle(path1: &Path, path2: &Path) -> f64 {
    let arr1: Vector3<f64> = path1.points[1] - path1.points[0];
    let arr2: Vector3<f64> = path2.points[1] - path2.points[0];

    let angle = (arr1.dot(&arr2) / (arr1.norm() * arr2.norm())).acos() * 180.0 / PI;
    return angle;
}
