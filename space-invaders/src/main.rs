// original sauce: https://github.com/bgaster/rusty-space-invaders/tree/master/src

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod interface;


use interface::create_interface;



fn main() {
    env_logger::init();

    let (event_loop, mut interface) = create_interface();

    println!("event loop:{:?}", event_loop);
    // println!("interface:{:?}", interface);
}