mod gcode_generator;
mod primitives;
use primitives::path::Path;
use primitives::point::Point;

fn main() {
    // print a rectangle
    let point1: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let point2: Point = Point {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let points: [Point; 2] = [point1, point2];

    let path: Path = Path {
        points: points,
        extrude: true,
    };
    let paths: &[Path] = &[path];

    let test: String = gcode_generator::from_paths(paths);

    println!("{:?}", test)
}
