// oonfig.rs
mod config;
use config::Config;
#[macro_use]
extern crate serde_derive;

// interface.rs
mod interface;

fn main() {
    env_logger::init();

    let mut config = Config:: new();

    println!("done");
}
