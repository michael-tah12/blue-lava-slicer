extern crate nalgebra as na;

use na::Vector3;

pub mod trivial;

// pub fn number_of_slices(triangles: &Vec<Triangle>) -> int {
//     let mut max_z: f64 = 0.0;
//     let mut min_z: f64 = 0.0;
//     for triangle in triangles.iter() {
//         let points = [tr.point1, tr.point2, tr.point3];
//         for point in points.iter() {
//             if point[2] > max_z {
//                 max_z = point[2];
//             }
//             if point[2] < max_z {
//                 max_z = point[2];
//             }
//         }
//     }
// }

fn slicing_point_exists(point1: &Vector3<f64>, point2: &Vector3<f64>, plane_z: &f64) -> bool {
    let zrange = point1[2]..=point2[2];
    if zrange.contains(plane_z) {
        return true;
    } else {
        return false;
    }
}

fn delete_null_paths(paths: &Vec<[Vector3<f64>; 2]>) -> Vec<[Vector3<f64>; 2]> {
    let mut filtered_paths: Vec<[Vector3<f64>; 2]> = Vec::new();
    for path in paths.iter() {
        if path[0] != path[1] {
            filtered_paths.push(*path)
        }
    }
    return filtered_paths;
}

fn delete_duplicate_paths(paths: &Vec<[Vector3<f64>; 2]>) -> Vec<[Vector3<f64>; 2]> {
    let mut filtered_paths: Vec<[Vector3<f64>; 2]> = Vec::new();
    for path in paths.iter() {
        if !filtered_paths.contains(&path) {
            filtered_paths.push(*path)
        }
    }
    return filtered_paths;
}
