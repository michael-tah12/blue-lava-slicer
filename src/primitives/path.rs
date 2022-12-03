use crate::primitives::point::Point;

pub struct Path {
    pub points: [Point; 2],
    pub extrude: bool,
}
