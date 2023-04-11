use crate::system;
use crate::color::Color;
use crate::geometry::{Point, Size};
use crate::sprite::Sprite;

pub struct Framebuffer {}

#[derive(Copy, Clone, Debug)]
pub enum DrawColorIndex {
    /// Foreground, fill
    Index1 = 0,
    /// Outline, background
    Index2 = 1,
    /// Additional color
    Index3 = 2,
    /// Another additional color
    Index4 = 3,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PaletteIndex {
    Transparent = 0,
    Palette1 = 1,
    Palette2 = 2,
    Palette3 = 3,
    Palette4 = 4,
}

#[allow(dead_code)]
impl Framebuffer {
    pub const unsafe fn new() -> Self {
        Self {}
    }

    pub fn get_size(&self) -> Size<u32> {
        Size::new(system::SCREEN_WIDTH, system::SCREEN_HEIGHT)
    }

    pub fn line(&self, start: Point<i32>, end: Point<i32>) {
        unsafe { system::line(start.x, start.y, end.x, end.y) }
    }

    pub fn line_horizontal(&self, start: Point<i32>, length: u32) {
        unsafe { system::hline(start.x, start.y, length) }
    }

    pub fn line_vertical(&self, start: Point<i32>, length: u32) {
        unsafe { system::vline(start.x, start.y, length) }
    }

    pub fn oval(&self, start: Point<i32>, size: Size<u32>) {
        unsafe { system::oval(start.x, start.y, size.width, size.height) }
    }

    pub fn rectangle(&self, start: Point<i32>, size: Size<u32>) {
        unsafe { system::rect(start.x, start.y, size.width, size.height) }
    }

    pub fn text(&self, text: &str, start: Point<i32>) {
        unsafe { system::textUtf8(text.as_ptr(), text.len(), start.x, start.y) }
    }

    pub fn sprite(&self, sprite: &Sprite, start: Point<i32>) {
        let bytes = sprite.bytes().as_ptr();
        let size = sprite.size();
        let flags = sprite.flags();
        unsafe {
            system::blit(bytes, start.x, start.y, size.width, size.height, flags as u32);
        }
    }

    pub fn set_draw_colors(&self, palettes: [PaletteIndex; 4]) {
        let mut draw_colors = 0u16;
        draw_colors |= (palettes[0] as u16) << DrawColorIndex::Index1.offset();
        draw_colors |= (palettes[1] as u16) << DrawColorIndex::Index2.offset();
        draw_colors |= (palettes[2] as u16) << DrawColorIndex::Index3.offset();
        draw_colors |= (palettes[3] as u16) << DrawColorIndex::Index4.offset();
        unsafe {
            *system::DRAW_COLORS = draw_colors;
        }
    }

    pub fn set_draw_color(&self, draw_color: DrawColorIndex, palette: PaletteIndex) {
        let mut draw_colors = unsafe { *system::DRAW_COLORS };
        // Clear bits for the draw color index
        draw_colors &= !(0b1111 << draw_color.offset());
        // Set proper bits for the draw color index
        draw_colors |= (palette as u16) << draw_color.offset();

        unsafe {
            *system::DRAW_COLORS = draw_colors;
        }
    }

    pub fn get_draw_colors(&self) -> [PaletteIndex; 4] {
        let draw_colors = unsafe { *system::DRAW_COLORS };
        [
            PaletteIndex::try_from(draw_colors & 0x000f >> DrawColorIndex::Index1.offset()).unwrap(),
            PaletteIndex::try_from(draw_colors & 0x00f0 >> DrawColorIndex::Index2.offset()).unwrap(),
            PaletteIndex::try_from(draw_colors & 0x0f00 >> DrawColorIndex::Index3.offset()).unwrap(),
            PaletteIndex::try_from(draw_colors & 0xf000 >> DrawColorIndex::Index4.offset()).unwrap(),
        ]
    }

    pub fn get_palette(&self) -> [Color; 4] {
        let palette = unsafe { *system::PALETTE };
        [
            Color::from(palette[0]),
            Color::from(palette[1]),
            Color::from(palette[2]),
            Color::from(palette[3]),
        ]
    }

    pub fn set_palette(&self, colors: [Color; 4]) {
        unsafe {
            *system::PALETTE = [
                colors[0].into(),
                colors[1].into(),
                colors[2].into(),
                colors[3].into(),
            ]
        }
    }
}

impl DrawColorIndex {
    fn offset(&self) -> u8 {
        match self {
            DrawColorIndex::Index1 => 0,
            DrawColorIndex::Index2 => 4,
            DrawColorIndex::Index3 => 8,
            DrawColorIndex::Index4 => 12
        }
    }
}

impl TryFrom<u16> for PaletteIndex {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PaletteIndex::Transparent),
            1 => Ok(PaletteIndex::Palette1),
            2 => Ok(PaletteIndex::Palette2),
            3 => Ok(PaletteIndex::Palette3),
            4 => Ok(PaletteIndex::Palette4),
            _ => Err(())
        }
    }
}