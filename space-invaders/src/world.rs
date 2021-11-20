// https://github.com/bgaster/rusty-space-invaders/blob/master/src/world.rs



use crate::Config;
use crate::sprite_helpers::SpriteSheet;

#[derive(Copy, Clone)]
pub enum GameState {
    Playing,
    Paused,
    Start,
    End
}

pub struct World {
    current_state: GameState,
    sprite_sheet: SpriteSheet
}

impl World {
    pub fn new() -> Self {
        World {
            current_state: GameState::Start,
            sprite_sheet: SpriteSheet::new("./assets/sprite-sheet.png")
        }
    }

    pub fn get_current_state(&self) -> GameState {
        self.current_state
    }

    pub fn get_sprite_sheet(&self) -> &SpriteSheet {
        &self.sprite_sheet
    }
}

pub fn initial_world_state(config: &Config) -> World {
    World::new()
}