// https://github.com/bgaster/rusty-space-invaders/blob/master/src/interface_desktop.in

use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use pixels::{Pixels, SurfaceTexture, wgpu::Surface}; 
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::dpi::LogicalSize;

const WIDTH :f64 = 480.0;
const HEIGHT :f64 = 460.0;

pub struct Interface {
    window: Window,
    input: WinitInputHelper, 
    pixels: Pixels,
    hidpi_factor: f64
}

impl Interface {
    pub fn request_redraw(&self){
        self.window.request_redraw();
    }

    pub fn handle_input(&mut self, event: Event<()>){
        let controls = {
            let mut left = self.input.key_held(VirtualKeyCode::Left);
            let mut right = self.input.key_held(VirtualKeyCode::Right);
            let mut fire = self.input.key_held(VirtualKeyCode::Space);
        };

        let direction = {

        };
    }

}

pub fn create_interface() -> (EventLoop<()>, Interface) {
        let event_loop = EventLoop::new();
        let window_size = LogicalSize::new(WIDTH, HEIGHT);
        let window = WindowBuilder::new()
            .with_title("Space Invadorz")
            .with_inner_size(window_size)
            .build(&event_loop)
            .unwrap();
        
        let hidpi_factor = window.scale_factor();
        let surface = Surface::create(&window);

        // let pixels = {

        // }

        (event_loop, Interface {
            window,
            hidpi_factor,
            input: WinitInputHelper::new(),
            pixels: Pixels::new(
                width: WIDTH, 
                height: HEIGHT, 
                surface: Surface::create(&window)
                surface_texture: SurfaceTexture::new(WIDTH as u32, HEIGHT as u32, surface)
            ).unwrap()
        });
    
    }