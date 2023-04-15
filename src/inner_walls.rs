use nalgebra::Vector3;

pub fn create_inner_paths_from_outer(outer_paths: &Vec<Vec<[Vector3<f64>; 2]>>) {
    for slice in outer_paths.iter() {
        println!("{:?}", slice)
    }
}
