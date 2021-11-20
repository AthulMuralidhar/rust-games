// https://github.com/bgaster/rusty-space-invaders/blob/master/src/config.rs

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: String,
    high_score: u32
}


impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            version: "0.1".to_string(),
            high_score: 0
        }
    }
}

impl Config {
    pub fn new()-> Self {
        confy::load("space-invaders").unwrap()
    }
}