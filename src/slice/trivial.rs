extern crate nalgebra as na;

use crate::read_stl::Triangle;
use crate::slice::{delete_duplicate_paths, delete_null_paths, slicing_point_exists};
use na::Vector3;

pub fn slice(triangles: &Vec<Triangle>, plane_z: f64) -> Vec<[Vector3<f64>; 2]> {
    let pp = Vector3::new(0.0, 0.0, plane_z);
    let vp = Vector3::new(0.0, 0.0, 1.0);
    let mut vv: Vector3<f64>;
    let mut sliceable: bool;
    let mut use_original_points = false;

    let mut slicing_points = [Vector3::new(0.0, 0.0, 0.0); 2];
    let mut slicing_points_collection: Vec<[Vector3<f64>; 2]> = Vec::new();
    let mut idx;

    for tr in triangles.iter() {
        idx = 0;
        let points = [
            [tr.points[0], tr.points[1]],
            [tr.points[0], tr.points[2]],
            [tr.points[1], tr.points[2]],
        ];
        for p in points {
            let mut pv = &p[0];
            let mut point2 = &p[1];
            if p[0][2] < p[1][2] {
                sliceable = true;
            } else if p[0][2] > p[1][2] {
                pv = &p[1];
                point2 = &p[0];
                sliceable = true;
            } else {
                sliceable = false;
                use_original_points = true;
            }
            if slicing_point_exists(pv, point2, &pp[2]) {
                if sliceable == true {
                    vv = *point2 - *pv;
                    slicing_points[idx] = calculate_slicing_point(&pv, &vv, &pp, &vp);
                    idx += 1;
                } else if use_original_points == true {
                    slicing_points[0] = p[0];
                    slicing_points[1] = p[1];
                    // println!("Use original points: {:?}, {:?}", p[0], p[1])
                }
            }
        }
        slicing_points_collection.push(slicing_points);
        slicing_points = [Vector3::new(0.0, 0.0, 0.0); 2]
    }
    return delete_duplicate_paths(&delete_null_paths(&slicing_points_collection));
}

fn calculate_slicing_point(
    pv: &Vector3<f64>,
    vv: &Vector3<f64>,
    pp: &Vector3<f64>,
    vp: &Vector3<f64>,
) -> Vector3<f64> {
    return pv + ((pp - pv).dot(&vp) / vv.dot(&vp)) * vv;
}
