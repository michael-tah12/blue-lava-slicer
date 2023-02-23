extern crate nalgebra as na;

use na::Vector3;
use std::fs;

use crate::read_stl::Triangle;

pub fn read(filename: &str) -> Vec<Triangle> {
    let raw = fs::read_to_string(filename).expect("");
    let lines: Vec<&str> = raw.split("\n").collect();
    let triangles: Vec<Triangle> = create_triangles(&lines);
    triangles
}

fn create_triangles(lines: &Vec<&str>) -> Vec<Triangle> {
    let mut triangles: Vec<Triangle> = Vec::new();
    for i in 1..lines.len() {
        let line = lines[i];
        if line.starts_with("facet") {
            let normal = Vector3::from_vec(line_to_vec(line, "facet normal "));
            let v1 = Vector3::from_vec(line_to_vec(lines[i + 2], "vertex "));
            let v2 = Vector3::from_vec(line_to_vec(lines[i + 3], "vertex "));
            let v3 = Vector3::from_vec(line_to_vec(lines[i + 4], "vertex "));
            triangles.push(Triangle {
                normal: normal,
                points: Vector3::new(v1, v2, v3),
            });
        }
    }
    return triangles;
}

fn line_to_vec(line: &str, prep: &str) -> Vec<f64> {
    let n: Vec<f64> = line.split(prep).collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()[0..3]
        .to_vec()
        .into_iter()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    return n;
}
