use bevy::prelude::{App, Camera2d, Commands, Component, Plugin, Startup};

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_main_camera);
    }
}

fn initialize_main_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}
