use core::ops::{Add, AddAssign};
use crate::geometry::Vector;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output=T>> Add<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign> AddAssign<Vector<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
