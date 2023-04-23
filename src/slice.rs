extern crate nalgebra as na;

use na::Vector3;

pub mod trivial;

#[derive(Debug, Clone, Copy)]
pub struct Path {
    pub points: [Vector3<f64>; 2],
    pub normal: Vector3<f64>,
}

impl Path {
    pub fn new() -> Self {
        return Self {
            points: [Vector3::zeros(), Vector3::zeros()],
            normal: Vector3::zeros(),
        };
    }
}

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

fn delete_null_paths(paths: &Vec<Path>) -> Vec<Path> {
    let mut filtered_paths: Vec<Path> = Vec::new();
    for path in paths.iter() {
        if path.points[0] != path.points[1] {
            filtered_paths.push(*path)
        }
    }
    return filtered_paths;
}

fn delete_duplicate_paths(paths: &Vec<Path>) -> Vec<Path> {
    let mut filtered_paths: Vec<Path> = Vec::new();
    for path in paths.iter() {
        let path_match: Vec<&Path> = filtered_paths
            .iter()
            .filter(|p| p.points == path.points)
            .collect();
        if path_match.is_empty() {
            filtered_paths.push(*path)
        }
    }
    return filtered_paths;
}
