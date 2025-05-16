mod camera;
mod default;
#[cfg(feature = "dev")]
mod development;
pub mod example;
mod physics;
mod window;

use bevy::prelude::{App, Plugin, PluginGroup};

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(default::DefaultPlugins);
        app.add_plugins(camera::CameraPlugin);
        app.add_plugins(physics::PhysicsPlugins);

        #[cfg(feature = "dev")]
        app.add_plugins(development::DevelopmentPlugin);
    }
}
