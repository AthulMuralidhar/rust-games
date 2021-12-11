// original sauce: https://github.com/bgaster/rusty-space-invaders/tree/master/src

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// #![feature(destructuring_assignment)]

mod interface;
mod config;
mod world;
mod render_helpers;
mod sprite_helpers;

use interface::create_interface;
use config::Config;
use world::initial_world_state;
use crate::render_helpers::{render_gameover, render_main_menu, render_system};
use crate::world::GameState;

/*  DEV NOTES
- the bevy branch containsspace invader immp;imentation using the bwvy engine
- the bevy engine is higly unstable and hence the examples used as reference almost
never work
- please prefer and consider sticking with the main implimenation WITHOUT the bevy engine
*/

fn main() {
    env_logger::init();

    let (event_loop, mut interface) = create_interface();

    // println!("event loop:{:?}", event_loop);
    // println!("interface:{:?}", interface);

    let mut config = Config::new();

    let mut world = initial_world_state(&config);

    event_loop.run(move |event, _, control_flow| {
        let current_state = world.get_current_state();

        if interface.should_render(&event) {
            match current_state {
                GameState::Playing => render_system(&world, &mut interface),
                GameState::Paused => render_system(&world, &mut interface),
                GameState::End => render_gameover(&world, &mut interface),
                _ => render_main_menu(&world, &mut interface)
            }
        }
    })

}
