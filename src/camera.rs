use bevy::prelude::{
    App, Camera2d, Commands, Component, Plugin, Startup,
};


#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;


pub struct MainCameraPlugin;


impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
    }
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}
