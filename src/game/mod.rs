//! Game-related plugin should be stored inside this module

use bevy::app::{App, Startup};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Plugin, Res, Sprite, default};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sprite);
    }
}

pub fn spawn_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bevy_logo = asset_server.load("textures/bevy.png");
    commands.spawn(Sprite {
        image: bevy_logo,
        custom_size: Some(Vec2::new(100., 100.)),
        ..default()
    });
}
