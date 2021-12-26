// https://mbuffett.com/posts/bevy-snake-tutorial/

mod snake;
mod constants;
mod window;

use snake::{Materials, spawn_snake, snake_movement};
use bevy::prelude::*;

fn main() {
    App::build()
    .add_startup_system(setup.system())
    .add_startup_stage("game setup", SystemStage::single(spawn_snake.system()))
    .add_system(snake_movement.system())
    .add_plugins(DefaultPlugins).run();
}


fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7,0.7,0.7).into())
    });
}