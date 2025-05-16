use crate::window;
use bevy::prelude::{App, DefaultPlugins as BevyDefaultPlugins, Plugin, PluginGroup};

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut App) {
        // Add bevy's DefaultPlugins with customization
        app.add_plugins(BevyDefaultPlugins.set(window::window_plugin()));
    }
}
