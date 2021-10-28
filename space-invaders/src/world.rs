use oorandom;
use spritesheet::SpriteSheet;

const RAND_SEED: i32 = 5;

pub enum GameState {
    Menu,
    Playing,
    Paused,
    End
}



// general utils
pub struct Timers {}
pub struct VFX {}



// VFX
pub struct Player_FX {
    bullet_explosion: String:,
    explosion: String,
}

//  entities
pub struct Player {
    score: u32,
    dead: bool,
    fx: Player_FX
}
pub struct AlienSwarm {}


//  world
pub struct World {
    state: GameState,
    random: oorandom::Rand32::new(RAND_SEED),
    sprite_sheet: SpriteSheet,
    menu: String,
    high_score: u32,
    player: Player,
    alien_swarm: AlienSwarm

}

