#![no_std]

extern crate alloc;

use alloc::string::ToString;
use core::fmt;
use crate::geometry::Size;

mod system;
pub mod application;
pub mod framebuffer;
pub mod geometry;
pub mod gamepad;
pub mod inputs;
pub mod sprite;
pub mod sound;

pub fn get_char_size() -> Size<u32> {
    Size::new(system::CHAR_WIDTH, system::CHAR_HEIGHT)
}

#[allow(dead_code)]
pub fn trace(msg: &str) {
    unsafe {
        system::traceUtf8(msg.as_ptr(), msg.len());
    }
}

pub fn _trace_args(args: fmt::Arguments) {
    let string = args.to_string();
    trace(&*string);
}

#[macro_export]
macro_rules! println {
    () => (
        $crate::trace("");
    );
    ($($arg:tt)*) => (
        $crate::_trace_args(format_args!($($arg)*))
    );
}
