// https://github.com/bgaster/rusty-space-invaders/blob/master/src/world.rs



use crate::Config;
use crate::sprite_helpers::{SpriteSheet, Sprite, SheetJSON};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameState {
    Playing,
    Paused,
    Start,
    End
}


pub struct World {
    current_state: GameState,
    sprite_sheet: SpriteSheet,
    splash: Sprite,
}

impl World {
    pub fn new(sprite_sheet: SpriteSheet, splash: Sprite) -> Self {
        World {
            current_state: GameState::Start,
            sprite_sheet,
            splash,
        }
    }

    pub fn get_current_state(&self) -> GameState {
        self.current_state
    }

    pub fn get_sprite_sheet(&self) -> &SpriteSheet {
        &self.sprite_sheet
    }

    pub fn get_splash_screen_sprite(&self) -> Sprite {
        self.splash.clone()
    }
    pub fn set_current_state(&mut self, state: GameState) {
        self.current_state = state
    } 

}

pub fn initial_world_state(config: &Config) -> World {
    let sprite_sheet = SpriteSheet::new("./assets/sprite-sheet.png");
    let sheet_json = SheetJSON::new("./assets/sprite-sheet.json");

    let s = sheet_json.frames.get("splash.png").unwrap();
    let splash = Sprite::new(
        s.frame.x as u32,
        s.frame.y as u32,
        s.frame.w as u32,
        s.frame.h as u32,
    );

    World::new(sprite_sheet, splash)
}