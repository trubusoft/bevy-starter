use crate::window;
use bevy::{
    log::{Level, LogPlugin},
    prelude::{App, DefaultPlugins as BevyDefaultPlugins, Plugin, PluginGroup, default},
};

pub struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    fn build(&self, app: &mut App) {
        // Add bevy's DefaultPlugins with customization
        app.add_plugins(
            BevyDefaultPlugins
                .set(window::window_plugin())
                .set(LogPlugin {
                    // show all dependency logs up to warn by default.
                    // as for bevy_starter show all level from trace to error.
                    // (change bevy_starter to your project name)
                    filter: "warn,bevy_starter=trace".into(),
                    level: Level::TRACE,
                    ..default()
                }),
        );
    }
}
