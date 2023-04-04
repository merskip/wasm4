use crate::geometry::Size;

pub struct Sprite {
    size: Size<u32>,
    flags: u32,
    bytes: &'static [u8],
}

impl Sprite {
    pub const fn new(size: Size<u32>, flags: u32, bytes: &'static [u8]) -> Self {
        Self { size, flags, bytes }
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }
    pub fn flags(&self) -> u32 {
        self.flags
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}