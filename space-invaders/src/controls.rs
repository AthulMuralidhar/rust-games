pub enum Direction {
    Still,
    Left,
    Right,
}
impl Default for Direction {
    fn default() -> Direction {
        Direction::Still
    }
}

pub struct Controls {
    pub direction: Direction,
    pub fire: bool
}

impl Default for Controls {
    fn default() -> Controls {
        Controls {
            direction: Direction::default(),
            fire: false
        }
    }
}