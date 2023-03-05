use crate::framebuffer::Framebuffer;
use crate::inputs::Inputs;

pub trait Application {
    /// Called at the start of the game, before the first update.
    fn start(inputs: &'static Inputs) -> Self;

    /// Called every frame, about 60 times per second.
    fn update(&mut self);

    /// Called after update
    fn render(&self, framebuffer: &'static Framebuffer);
}

#[macro_export]
macro_rules! main_application {
    ($application:ty) => {
        static mut MAIN_APPLICATION: core::mem::MaybeUninit<$application> = core::mem::MaybeUninit::uninit();
        static mut INPUTS: $crate::inputs::Inputs = unsafe { $crate::inputs::Inputs::new() };
        static mut FRAMEBUFFER: $crate::framebuffer::Framebuffer = unsafe { $crate::framebuffer::Framebuffer::new() };

        #[no_mangle]
        unsafe extern "C" fn start() {
            let application = <$application as $crate::application::Application>::start(&INPUTS);
            unsafe {
                MAIN_APPLICATION = core::mem::MaybeUninit::new(application);
            }
        }

        #[no_mangle]
        unsafe extern "C" fn update() {
            let application = unsafe { MAIN_APPLICATION.assume_init_mut() };
            application.update();
            unsafe { INPUTS.late_update() };
            application.render(&FRAMEBUFFER);
        }
    };
}