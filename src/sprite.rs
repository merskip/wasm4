use crate::geometry::Size;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Flags {
    BLIT_2BPP = 1,
    BLIT_1BPP = 0,
    BLIT_FLIP_X = 2,
    BLIT_FLIP_Y = 4,
    BLIT_ROTATE = 8,
}

pub struct Sprite {
    size: Size<u32>,
    flags: Flags,
    bytes: &'static [u8],
}

impl Sprite {
    pub const fn new(size: Size<u32>, flags: Flags, bytes: &'static [u8]) -> Self {
        Self { size, flags, bytes }
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }
    pub fn flags(&self) -> Flags {
        self.flags
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}