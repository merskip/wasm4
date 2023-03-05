#![no_std]

use crate::geometry::Size;

mod system;
pub mod application;
pub mod framebuffer;
pub mod geometry;
pub mod gamepad;
pub mod inputs;

pub fn get_char_size() -> Size<u32> {
    Size::new(system::CHAR_WIDTH, system::CHAR_HEIGHT)
}

#[allow(dead_code)]
pub fn trace(msg: &str) {
    unsafe {
        system::traceUtf8(msg.as_ptr(), msg.len());
    }
}
