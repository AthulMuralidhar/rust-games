// https://mbuffett.com/posts/bevy-snake-tutorial/
#![allow(dead_code)]

mod snake;
mod constants;
mod window;

use snake::{Materials, spawn_snake, snake_movement};
use bevy::prelude::*;
use window::{position_translation, size_scaling};
use bevy::render::pass::ClearColor;

fn main() {
    App::build()
    .insert_resource(WindowDescriptor {
        title: "Snaket bevy".to_string(),
        width: 500.0,
        height: 500.0,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
    .add_startup_system(setup.system())
    .add_startup_stage("game setup", SystemStage::single(spawn_snake.system()))
    .add_system(snake_movement.system())
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(position_translation.system())
            .with_system(size_scaling.system())
    )
    .add_plugins(DefaultPlugins).run();
}


fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7,0.7,0.7).into())
    });
}