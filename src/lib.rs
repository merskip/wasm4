#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use core::fmt;

use crate::color::Color;
use crate::framebuffer::Palette;

mod system;
pub mod application;
pub mod framebuffer;
pub mod gamepad;
pub mod inputs;
pub mod sprite;
pub mod audio;
pub mod color;
pub mod hsl_color;

pub fn get_char_width() -> u32 {
    system::CHAR_WIDTH
}

pub fn get_char_height() -> u32 {
    system::CHAR_HEIGHT
}

pub fn char_x_button() -> String {
    unsafe { String::from_utf8_unchecked(vec![system::CHAR_X_BUTTON]) }
}

pub fn char_y_button() -> String {
    unsafe { String::from_utf8_unchecked(vec![system::CHAR_Y_BUTTON]) }
}

pub fn char_left_arrow() -> String {
    unsafe { String::from_utf8_unchecked(vec![system::CHAR_LEFT_ARROW]) }
}

pub fn char_right_arrow() -> String {
    unsafe { String::from_utf8_unchecked(vec![system::CHAR_RIGHT_ARROW]) }
}

pub fn char_up_arrow() -> String {
    unsafe { String::from_utf8_unchecked(vec![system::CHAR_UP_ARROW]) }
}

pub fn char_down_arrow() -> String {
    unsafe { String::from_utf8_unchecked(vec![system::CHAR_DOWN_ARROW]) }
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

/// From https://wasm4.org/docs/guides/basic-drawing
pub const PALETTE_DEFAULT: Palette = [
    Color::from(0xe0f8cf),
    Color::from(0x86c06c),
    Color::from(0x306850),
    Color::from(0x071821)
];
