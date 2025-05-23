mod camera;
mod default;
#[cfg(feature = "development")]
mod development;
mod diagnostic;
mod game;
mod physics;
mod schedule;
mod state;
mod utils;
mod window;

use bevy::prelude::{App, Plugin};

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        // attach application's core plugin
        app.add_plugins(default::DefaultPlugin);
        app.add_plugins(camera::CameraPlugin);
        app.add_plugins(physics::PhysicsPlugin);
        app.add_plugins(state::StatePlugin);
        app.add_plugins(schedule::SchedulePlugin);

        // attach game's core plugin
        app.add_plugins(game::GamePlugin);

        #[cfg(feature = "development")]
        app.add_plugins(development::DevelopmentPlugin);
    }
}
