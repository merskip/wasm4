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
    pub width: u32,
    pub height: u32,
    pub flags: Flags,
    pub bytes: &'static [u8],
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