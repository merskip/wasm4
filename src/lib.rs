#![no_std]

mod system;
pub mod application;
pub mod framebuffer;
pub mod geometry;
pub mod gamepad;

pub const SYSTEM_CHAR_WIDTH: usize = 8;
pub const SYSTEM_CHAR_HEIGHT: usize = 8;

#[allow(dead_code)]
pub fn trace(msg: &str) {
    unsafe {
        system::traceUtf8(msg.as_ptr(), msg.len());
    }
}
