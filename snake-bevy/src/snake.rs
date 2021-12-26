use bevy::prelude::*;
use crate::window::{Position, Size};

pub struct SnakeHead;
pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
}

pub fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0,10.0)),
        ..Default::default()
    }).insert(SnakeHead)
    .insert(Position {x:3, y:3})
    .insert(Size::square(0.8));
}

pub fn snake_movement(
    keybord_input: Res<Input<KeyCode>>,
    mut head_position: Query<&mut Position, With<SnakeHead>>) {
    for mut pos in head_position.iter_mut() {
        if keybord_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keybord_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keybord_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keybord_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}