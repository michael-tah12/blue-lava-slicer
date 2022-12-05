mod gcode_generator;
mod primitives;

use primitives::path::Path;

fn main() {
    let mut gcode_vec: Vec<String> = Vec::new();

    // print a rectangle
    let point1: [f64; 3] = [0.0, 0.0, 0.0];
    let point2: [f64; 3] = [1.0, 0.0, 0.0];

    let path1: Path = Path {
        target: point1,
        extrude: false,
    };

    let path2: Path = Path {
        target: point2,
        extrude: true,
    };
    let paths: &[Path] = &[path1, path2];

    let test: String = gcode_generator::from_paths(paths);
    gcode_vec.push(test);

    print!("{}", gcode_vec.join("\n"))
}
