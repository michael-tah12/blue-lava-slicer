use crate::primitives::path::Path;

pub fn from_paths(paths: &Vec<Path>) -> String {
    let mut gcode: String = String::from("");
    let mut relative_movement = false;
    for path in paths.iter() {
        let mut move_type = "G0";
        if path.extrude {
            move_type = "G1";
        }
        if path.relative != relative_movement {
            relative_movement = path.relative;
            let mut movement_type = "G90\n";
            if path.relative {
                movement_type = "G91\n"
            }
            gcode.push_str(movement_type)
        }
        let line = format!(
            "{} X{} Y{} Z{}\n",
            move_type, path.target[0], path.target[1], path.target[2]
        );
        gcode.push_str(line.as_str());
    }
    return gcode;
}
