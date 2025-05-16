use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::input_just_pressed,
    prelude::{App, IntoScheduleConfigs, KeyCode, Plugin, ResMut, UiDebugOptions, Update},
};
use log::info;

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

/// Anything related to the development cycle should be added on this plugin.
///
/// Note:
/// 1. This plugin is designed not to be included on release.
/// 2. The [LogDiagnosticsPlugin] itself only logs diagnostic to the console,
///    while diagnostic is sent by other module such as [FrameTimeDiagnosticsPlugin].
///    Adding arbitrary diagnostic plugin without adding [LogDiagnosticsPlugin]
///    won't print anything to the console
pub struct DevelopmentPlugin;

impl Plugin for DevelopmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LogDiagnosticsPlugin::default());
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
    }
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
    info!("Toggling UiDebugOptions");
}
