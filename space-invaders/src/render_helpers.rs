// https://github.com/bgaster/rusty-space-invaders/blob/master/src/renderer.rs


use crate::world::World;
use crate::interface::Interface;

pub fn render_main_menu(world: &World, interface: &mut Interface) {
    // let mut frame_buffer = interface.framebuffer();

    interface.draw_call();

    let sprite_sheet = world.get_sprite_sheet();

}

pub fn render_gameover(world: &World, interface: &mut Interface) {

}

pub fn render_system(world: &World, interface: &mut Interface) {

}