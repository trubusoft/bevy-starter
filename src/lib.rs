mod camera;
#[cfg(feature = "dev")]
mod development;
pub mod example;
mod physics;
mod window;

use bevy::prelude::{App, DefaultPlugins, Plugin, PluginGroup};

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        // DefaultPlugins and its customization
        app.add_plugins(DefaultPlugins.set(window::window_plugin()));

        // Project-related plugins
        app.add_plugins(camera::MainCameraPlugin);
        app.add_plugins(physics::PhysicsPlugins);

        #[cfg(feature = "dev")]
        app.add_plugins(development::DevelopmentPlugin);
    }
}
