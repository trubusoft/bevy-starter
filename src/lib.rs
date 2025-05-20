mod camera;
mod default;
#[cfg(feature = "dev")]
mod development;
mod diagnostic;
pub mod example;
mod physics;
mod utils;
mod window;

use bevy::prelude::{App, Plugin};

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(default::DefaultPlugin);
        app.add_plugins(camera::CameraPlugin);
        app.add_plugins(physics::PhysicsPlugin);

        #[cfg(feature = "dev")]
        app.add_plugins(development::DevelopmentPlugin);
    }
}
