use libm::sqrtf;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vector<f32> {
    /// Returns the length of this vector
    pub fn magnitude(&self) -> f32 {
        sqrtf(self.x * self.x + self.y * self.y)
    }

    pub fn normalized(&self) -> Self {
        let length = self.magnitude();
        if length == 0.0 {
            return *self;
        }
        let inverted_length = 1.0 / self.magnitude();
        Vector {
            x: self.x * inverted_length,
            y: self.y * inverted_length,
        }
    }
}