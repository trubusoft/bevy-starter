mod camera;
#[cfg(feature = "dev")]
mod development;
pub mod example;

use bevy::prelude::{App, DefaultPlugins, Plugin};

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            camera::MainCameraPlugin
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(development::DevelopmentPlugin);
    }
}
