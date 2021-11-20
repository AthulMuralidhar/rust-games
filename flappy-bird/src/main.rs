// https://github.com/TanTanDev/flappy_bevy/blob/master/src/main.rs

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod screen;

use bevy::prelude::*;
use crate::screen::ScreenPlugin;


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenPlugin)
        .run();
}
