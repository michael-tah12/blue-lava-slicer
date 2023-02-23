use std::fs;

mod gcode_generator;
mod primitives;

use primitives::path::Path;

fn main() {
    let mut gcode_vec: Vec<String> = Vec::new();

    // read header
    let header: String = fs::read_to_string("header.gcode").expect("");
    gcode_vec.push(header);

    // print a rectangle
    let point1: [f64; 3] = [20.0, 20.0, 0.0];
    let point2: [f64; 3] = [10.0, 0.0, 0.0];
    let point3: [f64; 3] = [0.0, 10.0, 0.0];
    let point4: [f64; 3] = [0.0, 0.0, 0.0];
    let point5: [f64; 3] = [10.0, 0.0, 0.0];

    let path1: Path = Path {
        target: point1,
        extrude: false,
        relative: false
    };

    let path2: Path = Path {
        target: point2,
        extrude: true,
        relative: true
    };

    let path3: Path = Path {
        target: point3,
        extrude: true,
        relative: true
    };

    let path4: Path = Path {
        target: point4,
        extrude: false,
        relative: false
    };

    let mut paths: Vec<Path> = vec![path1, path2, path3, path4];

    let path5: Path = Path {
        target: point5,
        extrude: false,
        relative: true
    };
    paths.push(path5);

    let test: String = gcode_generator::from_paths(&paths);
    gcode_vec.push(test);

    print!("{}", gcode_vec.join("\n"))
}
