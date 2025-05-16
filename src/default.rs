use crate::window;
use bevy::prelude::{App, DefaultPlugins as BevyDefaultPlugins, Plugin, PluginGroup};

pub struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    fn build(&self, app: &mut App) {
        // Add bevy's DefaultPlugins with customization
        app.add_plugins(BevyDefaultPlugins.set(window::window_plugin()));
    }
}
