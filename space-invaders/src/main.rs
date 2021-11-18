// original sauce: https://github.com/bgaster/rusty-space-invaders/tree/master/src

mod interface;


use interface::create_interface;

fn main() {
    env_logger::init();

    let (event_loop, mut interface) = create_interface("Space invaders");
}