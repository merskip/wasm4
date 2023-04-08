use crate::geometry::{Point, Size};

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Rect {
    pub origin: Point<i32>,
    pub size: Size<u32>,
}

impl Rect {
    pub const fn new(origin: Point<i32>, size: Size<u32>) -> Self {
        Self { origin, size }
    }

    pub fn centered(&self, size: Size<u32>) -> Self {
        Self {
            origin: Point::new(
                self.origin.x + ((self.size.width - size.width) / 2) as i32,
                self.origin.y + ((self.size.height - size.height) / 2) as i32,
            ),
            size,
        }
    }
}
