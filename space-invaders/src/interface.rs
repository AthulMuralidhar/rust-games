// https://github.com/bgaster/rusty-space-invaders/blob/master/src/interface_desktop.in

use winit::window::Window;
use winit_input_helper::WinitInputHelper;
use pixels::Pixels; 
use winit::event::Event;
use winit::event_loop::EventLoop;

pub struct Inteface {
    window: Window,
    input: WinitInputHelper, 
    pixels: Pixels
}

impl Inteface {
    pub fn request_redraw(&self){
        self.window.request_redraw();
    }

    pub fn handle_input(&mut self, event: Event){
        let controls = {
            let mut left = self.input.key_held(VirtualKeyCode::Left);
            let mut right = self.input.key_held(VirtualKeyCode::Right);
            let mut fire = self.input.key_held(VirtualKeyCode::Space);
        }

        let direction = {

        }
    }



    pub fn create_interface(){
        let event_loop = EventLoop::new();
    }