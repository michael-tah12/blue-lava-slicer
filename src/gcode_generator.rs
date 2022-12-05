use crate::primitives::path::Path;

pub fn from_paths(paths: &[Path]) -> String {
    let mut gcode: String = String::from("");
    for path in paths.iter() {
        let mut move_type = "G0";
        if path.extrude {
            move_type = "G1"
        }
        let line = format!(
            "{} X{} Y{} Z{}\n",
            move_type, path.target[0], path.target[1], path.target[2]
        );
        gcode.push_str(line.as_str())
    }
    return gcode;
}
