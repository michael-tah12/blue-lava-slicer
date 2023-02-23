extern crate nalgebra as na;

pub mod read_stl_ascii;

use na::SVector;

#[derive(Debug)]
pub struct Triangle {
    pub normal: SVector<f64, 3>,
    pub points: SVector<SVector<f64, 3>, 3>,
}
