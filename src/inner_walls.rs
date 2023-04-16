use nalgebra::Vector3;

pub fn create_inner_paths_from_outer(
    outer_paths: &Vec<Vec<[Vector3<f64>; 2]>>,
) -> Vec<Vec<[Vector3<f64>; 2]>> {
    let mut shrinked_slices: Vec<Vec<[Vector3<f64>; 2]>> = Vec::new();
    for slice in outer_paths.iter() {
        println!("{:?}\n", slice);
        let shrinked_slice = shrink_slice(slice, 0.9);
        println!("{:?}\n", shrinked_slice);
        shrinked_slices.push(shrinked_slice);
        // for (i, path) in slice.repeat(2).iter().enumerate() {
        //     println!("{:?}", path);
        //     if i == slice.len() - 1 {
        //         break;
        //     }
        // }
    }
    return shrinked_slices;
}

fn shrink_slice(slice: &Vec<[Vector3<f64>; 2]>, factor: f64) -> Vec<[Vector3<f64>; 2]> {
    let mut shrinked_slice: Vec<[Vector3<f64>; 2]> = Vec::new();
    for (i, path) in slice.iter().enumerate() {
        let start: Vector3<f64>;
        if i == 0 {
            start = path[0]
        } else {
            start = shrinked_slice[i - 1][1]
        }
        let path_length = (path[1] - start).norm();
        let direction = (path[1] - start) / path_length;
        let new_end = direction * path_length * factor;
        shrinked_slice.push([start, start + new_end])
    }
    return shrinked_slice;
    //for path in slice.iter() {}
}
