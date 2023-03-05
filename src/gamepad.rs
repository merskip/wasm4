use core::ops::BitAnd;
use crate::system;

#[derive(Eq, PartialEq)]
pub struct Gamepad {
    address: *const u8,
    last_state: u8
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum GamepadButton {
    ButtonX = system::BUTTON_1,
    ButtonY = system::BUTTON_2,
    DPadLeft = system::BUTTON_LEFT,
    DPadRight = system::BUTTON_RIGHT,
    DPadUp = system::BUTTON_UP,
    DPadDown = system::BUTTON_DOWN,
}

#[allow(dead_code)]
impl Gamepad {
    pub(crate) const unsafe fn new(address: *const u8) -> Self {
        Gamepad { address, last_state: 0 }
    }
}

impl Gamepad {
    pub(crate) unsafe fn late_update(&mut self) {
        unsafe {
            self.last_state = *self.address;
        }
    }

    pub fn is_pressed(&self, button: GamepadButton) -> bool {
        let state = unsafe { *self.address };
        !Self::is_pressing(self.last_state, &button) && Self::is_pressing(state, &button)
    }

    pub fn is_held(&self, button: GamepadButton) -> bool {
        let state = unsafe { *self.address };
        Self::is_pressing(state, &button)
    }

    pub fn is_released(&self, button: GamepadButton) -> bool {
        let state = unsafe { *self.address };
        Self::is_pressing(self.last_state, &button) && !Self::is_pressing(state, &button)
    }

    fn is_pressing(state: u8, button: &GamepadButton) -> bool {
        state.bitand(*button as u8) != 0
    }
}