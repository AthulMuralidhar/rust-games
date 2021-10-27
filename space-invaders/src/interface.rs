// source
// https://github.com/bgaster/rusty-space-invaders/blob/master/src/interface_desktop.in

use gilrs::{GamepadId, Gilrs};
use pixels::Pixels;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

pub struct Interface {
    pub window: Window,
    pub hidpi_factor: f64,
    pub input: WinitInputHelper,
    pub gilrs: Gilrs,
    pub gamepad: GamepadId,
    pub pixels: Pixels
}

impl Interface {
    pub fn requet_redraw(&self) {
        self.window.request_redraw();
    }
}