use crate::geometry::{Point, Size};

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Rect {
    pub origin: Point<i32>,
    pub size: Size<u32>,
}

impl Rect {
    pub const fn new(origin: Point<i32>, size: Size<u32>) -> Self {
        Self { origin, size }
    }
}
