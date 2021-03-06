use bevy::prelude::*;
use crate::constants::{Materials, ARENA_HEIGHT, ARENA_WIDTH};
use crate::window::{Position, Size};
use rand::prelude::*;

pub struct Food;

pub fn food_spawner(  mut commands: Commands,
    materials: Res<Materials>) {
   commands.spawn_bundle(SpriteBundle {
       material: materials.food_material.clone(),
       ..Default::default()
   })
   .insert(Food)
   .insert(Position {
       x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
       y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
   })
   .insert(Size::square(0.8));
}