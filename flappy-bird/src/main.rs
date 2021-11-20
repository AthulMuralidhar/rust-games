// https://github.com/TanTanDev/flappy_bevy/blob/master/src/main.rs

#![allow(unused_imports)]

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::prelude::Camera2dComponents;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system()
        .run();
}

fn setup(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    commands.spawn(Camera2dComponents::default());


}