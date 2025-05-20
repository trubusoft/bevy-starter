// Disable console on Windows for non development builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::App;
use bevy_starter::ApplicationPlugin;

fn main() {
    App::new().add_plugins(ApplicationPlugin).run();
}
