use bevy::prelude::*;
use crate::window::{Position, Size, Direction};
use crate::constants::Materials;

pub struct SnakeHead {
    direction: Direction,
}

pub fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0,10.0)),
        ..Default::default()
    }).insert(SnakeHead {
        direction: Direction::Up,
    })
    .insert(Position {x:3, y:3})
    .insert(Size::square(0.8));
}

pub fn snake_movement(
    mut heads: Query<(&mut Position, &SnakeHead)>
) {
    if let Some((mut head_pos, head)) = heads.iter_mut().next() {
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        }
    }

}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

pub fn snake_movement_input(
    keybord_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keybord_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keybord_input.pressed(KeyCode::Down) {
            Direction::Down
        }else if keybord_input.pressed(KeyCode::Up) {
            Direction::Up
        }else if keybord_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}


pub struct SnakeSegemt;

#[derive(Default)]
pub struct SnakeSegemts(Vec<Entity>);

pub fn spawn_segment(
    
){

}