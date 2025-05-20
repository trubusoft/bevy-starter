use bevy::app::App;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::Plugin;

/// Anything related to diagnostic should be added on this plugin.
///
/// Maybe useful for both development and release on a case-by-case basis.
///
/// Note: The [LogDiagnosticsPlugin] itself only logs diagnostic to the console,
/// while diagnostic is sent by other module such as [FrameTimeDiagnosticsPlugin].
/// Adding arbitrary diagnostic plugin without adding [LogDiagnosticsPlugin]
/// won't print anything to the console
pub struct DiagnosticPlugin;

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LogDiagnosticsPlugin::default());
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    }
}
