
// source
// https://github.com/bgaster/rusty-space-invaders/

// oonfig.rs
mod config;
use config::Config;
#[macro_use]
extern crate serde_derive;

// interface.rs
mod interface;

// world.rs


fn main() {
    env_logger::init();

    let _config = Config:: new();

    println!("compiles!");
}
