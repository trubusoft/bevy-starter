use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::input_just_pressed,
    prelude::{App, IntoScheduleConfigs, KeyCode, Plugin, ResMut, UiDebugOptions, Update},
};
use log::info;

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

/// Anything related to the development cycle should be added on this plugin.
///
/// Note: This plugin is designed not to be included on release.
pub struct DevelopmentPlugin;

impl Plugin for DevelopmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_ui_debug_option.run_if(input_just_pressed(TOGGLE_KEY)),
        );
    }
}

fn toggle_ui_debug_option(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
    info!("Toggling UiDebugOptions");
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::ButtonInput;

    #[test]
    fn toggle_ui_debug_option_test() {
        let mut app = App::new();
        app.insert_resource(UiDebugOptions::default());

        let ui_debug_options = app.world().get_resource::<UiDebugOptions>();
        assert!(ui_debug_options.is_some());
        assert!(!ui_debug_options.unwrap().enabled);

        app.add_systems(Update, toggle_ui_debug_option);
        app.update();

        let ui_debug_options = app.world().get_resource::<UiDebugOptions>();
        assert!(ui_debug_options.is_some());
        assert!(ui_debug_options.unwrap().enabled);
    }

    #[test]
    fn toggle_ui_debug_option_with_key_press() {
        let mut app = App::new();
        app.insert_resource(UiDebugOptions::default());
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.add_systems(
            Update,
            toggle_ui_debug_option.run_if(input_just_pressed(TOGGLE_KEY)),
        );

        // initial state
        let ui_debug_options = app.world().get_resource::<UiDebugOptions>();
        assert!(ui_debug_options.is_some());
        assert!(!ui_debug_options.unwrap().enabled);

        // no key press
        app.update();
        let ui_debug_options = app.world().get_resource::<UiDebugOptions>();
        assert!(ui_debug_options.is_some());
        assert!(!ui_debug_options.unwrap().enabled);

        // with key press
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(TOGGLE_KEY);
        app.insert_resource(input);
        app.update();

        let ui_debug_options = app.world().get_resource::<UiDebugOptions>();
        assert!(ui_debug_options.is_some());
        assert!(ui_debug_options.unwrap().enabled);
    }
}
