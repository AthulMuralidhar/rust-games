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
    mut head_position: Query<&mut Transform, With<SnakeHead>>) {
    for mut transform in head_position.iter_mut() {
        if keybord_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.0;
        }
        if keybord_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.0;
        }
        if keybord_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.0;
        }
        if keybord_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.0;
        }
    }
}