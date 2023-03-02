#![no_std]

mod system;
pub mod application;
pub mod framebuffer;
pub mod geometry;
pub mod gamepad;

#[allow(dead_code)]
pub fn trace(msg: &str) {
    unsafe {
        system::traceUtf8(msg.as_ptr(), msg.len());
    }
}
