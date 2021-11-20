
use crate::{AssetServer, Commands};
use bevy::prelude::*;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin{
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_verver: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let start_texture_handle = asset_verver.load("assets/SpaceToStart.png").unwrap();

}