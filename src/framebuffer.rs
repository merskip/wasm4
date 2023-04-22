use crate::color::Color;
use crate::sprite::Sprite;
use crate::system;

pub struct Framebuffer {}

#[derive(Copy, Clone, Debug)]
enum DrawColorIndex {
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

pub type Palette = [Color; 4];

#[allow(dead_code)]
impl Framebuffer {
    pub const unsafe fn new() -> Self {
        Self {}
    }

    pub fn get_screen_width(&self) -> u32 {
        system::SCREEN_WIDTH
    }

    pub fn get_screen_height(&self) -> u32 {
        system::SCREEN_HEIGHT
    }

    pub fn line(&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
        unsafe { system::line(start_x, start_y, end_x, end_y) }
    }

    pub fn line_horizontal(&self, start_x: i32, start_y: i32, length: u32) {
        unsafe { system::hline(start_x, start_y, length) }
    }

    pub fn line_vertical(&self, start_x: i32, start_y: i32, length: u32) {
        unsafe { system::vline(start_x, start_y, length) }
    }

    pub fn oval(&self, start_x: i32, start_y: i32, width: u32, height: u32) {
        unsafe { system::oval(start_x, start_y, width, height) }
    }

    pub fn rectangle(&self, start_x: i32, start_y: i32, width: u32, height: u32) {
        unsafe { system::rect(start_x, start_y, width, height) }
    }

    pub fn text(&self, text: &str, start_x: i32, start_y: i32) {
        unsafe { system::textUtf8(text.as_ptr(), text.len(), start_x, start_y) }
    }

    pub fn sprite(&self, sprite: &Sprite, start_x: i32, start_y: i32) {
        unsafe {
            system::blit(sprite.bytes.as_ptr(), start_x, start_y, sprite.width, sprite.height, sprite.flags as u32);
        }
    }

    pub fn set_draw_colors(&self, palettes: [Option<PaletteIndex>; 4]) {
        let mut draw_colors = unsafe { *system::DRAW_COLORS };
        let mut set_draw_color = |draw_color: DrawColorIndex, palette| {
            if let Some(palette) = palette {
                draw_colors &= !(0b1111 << draw_color.offset());
                draw_colors |= (palette as u16) << draw_color.offset();
            }
        };
        set_draw_color(DrawColorIndex::Index1, palettes[0]);
        set_draw_color(DrawColorIndex::Index2, palettes[1]);
        set_draw_color(DrawColorIndex::Index3, palettes[2]);
        set_draw_color(DrawColorIndex::Index4, palettes[3]);
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

    pub fn get_palette(&self) -> Palette {
        let palette = unsafe { *system::PALETTE };
        [
            Color::from(palette[0]),
            Color::from(palette[1]),
            Color::from(palette[2]),
            Color::from(palette[3]),
        ]
    }

    pub fn set_palette(&self, palette: Palette) {
        unsafe {
            *system::PALETTE = [
                palette[0].into(),
                palette[1].into(),
                palette[2].into(),
                palette[3].into(),
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