use crate::primitives::path::Path;

pub fn from_paths(paths: &[Path]) -> String {
    let mut gcode: String = String::from("");
    for path in paths.iter() {
        for point in path.points.iter() {
            let line = format!("G0 X{} Y{} Z{}\n",point.x,point.y,point.z);
            gcode.push_str(line.as_str())
        }
    }
    return gcode;
}
