// https://github.com/bgaster/rusty-space-invaders/blob/master/src/renderer.rs


use crate::world::World;
use crate::interface::Interface;

pub fn render_main_menu(world: &World, interface: &mut Interface) {
    let mut frame_buffer = interface.framebuffer();
    let sprite_sheet = world.get_sprite_sheet();

    world.get_splash_screen_sprite().render(0,0, sprite_sheet, &mut frame_buffer);
    interface.draw_call();


}

pub fn render_gameover(world: &World, interface: &mut Interface) {

}

pub fn render_system(world: &World, interface: &mut Interface) {

}