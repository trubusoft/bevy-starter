mod camera;
mod default;
#[cfg(feature = "development")]
mod development;
mod diagnostic;
pub mod example;
mod physics;
mod schedule;
mod state;
mod utils;
mod window;

use bevy::prelude::{App, IntoScheduleConfigs, Plugin, SystemSet, Update};
use schedule::ApplicationSchedule;

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(default::DefaultPlugin);
        app.add_plugins(camera::CameraPlugin);
        app.add_plugins(physics::PhysicsPlugin);
        app.add_plugins(state::StatePlugin);

        // attach application's internal system set to bevy's Update
        app.configure_sets(
            Update,
            (
                ApplicationSchedule::TickTimers,
                ApplicationSchedule::RecordInput,
                ApplicationSchedule::Update,
            )
                .chain(),
        );

        #[cfg(feature = "development")]
        app.add_plugins(development::DevelopmentPlugin);
    }
}
