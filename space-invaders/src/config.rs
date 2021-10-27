#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub high_score: u32
}

impl Config {
    pub fn new() -> Self {
        confy::load("space-invaders").unwrap()
    }
}

impl ::std::default::Default for Config{
    fn default() -> Self {
        Self {
            version: "0.1".into(),
            high_score: 0
        }
    }
}