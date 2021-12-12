// original sauce: https://github.com/bgaster/rusty-space-invaders/tree/master/src

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
// #![feature(destructuring_assignment)]

mod interface;
mod config;
mod world;
mod render_helpers;
mod sprite_helpers;
mod controls;
mod constants;
mod frame;

use interface::create_interface;
use config::Config;
use world::initial_world_state;
use crate::render_helpers::{render_gameover, render_main_menu, render_system};
use crate::world::GameState;
use log::{info};

/*  DEV NOTES
- the bevy branch containsspace invader immp;imentation using the bwvy engine
- the bevy engine is higly unstable and hence the examples used as reference almost
never work
- please prefer and consider sticking with the main implimenation WITHOUT the bevy engine
*/

fn main() {
    env_logger::init();

    let (event_loop, mut interface) = create_interface();
    info!("Game interface created");

    let mut config = Config::new();
    let mut world = initial_world_state(&config);
    info!("Game config loadded to the world");

    // main game loop
    event_loop.run(move |event, _, control_flow| {
        info!("Enterring main game loop");

        let current_state = world.get_current_state();
        info!("Current game state: {:?}", current_state);

        if interface.render(&event) {
            match current_state {
                GameState::Playing => render_system(&world, &mut interface),
                GameState::Paused => render_system(&world, &mut interface),
                GameState::End => render_gameover(&world, &mut interface),
                _ => render_main_menu(&world, &mut interface)
            }
        }

            
        let (should_exit, controls) = interface.handle_input(event);

        match current_state {
            GameState::Playing => {
                // player_control_system(&mut world, controls);
                // bullet_control_system(&mut world);

                // alien_control_system(&mut world);

                // ship_control_system(&mut world);
                // bullet_collision_system(&mut world);
                // world.update();
            }
            _ => { // GameState::Start
                if let Some(control) = controls {
                    if control.fire {
                        world.set_current_state(GameState::Playing);
                    }
                }
            }
        }


        interface.request_redraw();
    });

}
