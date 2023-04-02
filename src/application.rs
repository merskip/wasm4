use crate::framebuffer::Framebuffer;
use crate::inputs::Inputs;

pub trait Application {
    /// Called at the start of the game, before the first update.
    fn start() -> Self;

    /// Called every frame, about 60 times per second.
    fn update(&mut self, inputs: &Inputs);

    /// Called after update
    fn render(&self, framebuffer: &Framebuffer);
}

static mut INPUTS: Inputs = unsafe { Inputs::new() };
static mut FRAMEBUFFER: Framebuffer = unsafe { Framebuffer::new() };

pub unsafe fn get_shared_inputs() -> &'static mut Inputs {
    &mut INPUTS
}

pub unsafe fn get_shared_framebuffer() -> &'static mut Framebuffer {
    &mut FRAMEBUFFER
}

#[macro_export]
macro_rules! main_application {
    ($application:ty) => {
        static mut MAIN_APPLICATION: core::mem::MaybeUninit<$application> = core::mem::MaybeUninit::uninit();

        #[no_mangle]
        unsafe extern "C" fn start() {
            let application = <$application as $crate::application::Application>::start();
            unsafe {
                MAIN_APPLICATION = core::mem::MaybeUninit::new(application);
            }
        }

        #[no_mangle]
        unsafe extern "C" fn update() {
            let application = unsafe { MAIN_APPLICATION.assume_init_mut() };

            let inputs = unsafe { $crate::application::get_shared_inputs() };
            application.update(inputs);
            unsafe { inputs.late_update() };

            let framebuffer = unsafe { $crate::application::get_shared_framebuffer() };
            application.render(framebuffer);
        }
    };
}