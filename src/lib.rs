mod development;
pub mod example;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{App, DefaultPlugins, Plugin};

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);

        // Enable several development tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins((
            development::DevelopmentPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ));
    }
}
