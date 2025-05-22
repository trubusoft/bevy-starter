use bevy::app::App;
use bevy::prelude::{AppExtStates, Plugin, States};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ApplicationState>();
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum ApplicationState {
    #[default]
    InitialScreen,
    // and so on.
    // Loading,
    // MainMenu,
    // HowToPlay,
    // Setting,
    // and so on.
    // Level1,
    // Level2,
    // Level3,
}
