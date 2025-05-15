use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::{App, IntoScheduleConfigs, KeyCode, Plugin, ResMut, UiDebugOptions, Update};

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

pub struct DevelopmentPlugin;

impl Plugin for DevelopmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
    }
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
