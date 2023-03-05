use crate::gamepad::Gamepad;
use crate::system;

pub struct Inputs {
    pub gamepad1: Gamepad,
    pub gamepad2: Gamepad,
    pub gamepad3: Gamepad,
    pub gamepad4: Gamepad,
}

impl Inputs {
    pub const unsafe fn new() -> Self {
        Self {
            gamepad1: unsafe { Gamepad::new(system::GAMEPAD1) },
            gamepad2: unsafe { Gamepad::new(system::GAMEPAD2) },
            gamepad3: unsafe { Gamepad::new(system::GAMEPAD3) },
            gamepad4: unsafe { Gamepad::new(system::GAMEPAD4) },
        }
    }

    pub unsafe fn late_update(&mut self) {
        self.gamepad1.late_update();
        self.gamepad2.late_update();
        self.gamepad3.late_update();
        self.gamepad4.late_update();
    }
}