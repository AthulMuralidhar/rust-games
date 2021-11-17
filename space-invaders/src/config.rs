// https://github.com/bgaster/rusty-space-invaders/blob/master/src/config.rs

pub struct Config {
    version: String,
    high_score: u32
}


impl ::std::default::Default for Config {
    fn default() -> Self {
        version: "0.1",
        high_score: 0
    }
}