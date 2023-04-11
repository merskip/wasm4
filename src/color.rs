use core::fmt::{Debug, Formatter};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[allow(dead_code)]
impl Color {
    pub const BLACK: Color = Self::new(0x00, 0x00, 0x00);
    pub const WHITE: Color = Self::new(0xff, 0xff, 0xff);
    pub const RED: Color = Self::new(0xff, 0x00, 0x00);
    pub const GREEN: Color = Self::new(0x00, 0xff, 0x00);
    pub const BLUE: Color = Self::new(0x00, 0x00, 0xff);
    pub const YELLOW: Color = Self::new(0xff, 0xff, 0x00);
    pub const CYAN: Color = Self::new(0x00, 0xff, 0xff);
    pub const MAGENTA: Color = Self::new(0xff, 0x00, 0xff);

    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color::new(
            ((value & 0xff0000) >> 16) as u8,
            ((value & 0x00ff00) >> 8) as u8,
            ((value & 0x0000ff) >> 0) as u8,
        )
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        ((self.red as u32) << 16)
            | ((self.green as u32) << 8)
            | ((self.blue as u32) << 0)
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}
