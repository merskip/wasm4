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
    width: u32,
    height: u32,
    flags: Flags,
    bytes: &'static [u8],
}

impl Sprite {
    pub const fn new(width: u32, height: u32, flags: Flags, bytes: &'static [u8]) -> Self {
        Self { width, height, flags, bytes }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn flags(&self) -> Flags {
        self.flags
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            flags: Flags::BLIT_1BPP,
            bytes: &[],
        }
    }
}