use crate::geometry::{Point, Size};
use crate::system;

pub struct Framebuffer {}

#[allow(dead_code)]
impl Framebuffer {
    pub unsafe fn new() -> Self {
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

    pub fn set_color(&self, color_index: u16) {
        unsafe { *system::DRAW_COLORS = color_index }
    }
}