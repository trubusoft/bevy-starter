mod camera;
mod default;
#[cfg(feature = "development")]
mod development;
mod diagnostic;
pub mod example;
mod physics;
mod utils;
mod window;

use bevy::prelude::{App, IntoScheduleConfigs, Plugin, SystemSet, Update};

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(default::DefaultPlugin);
        app.add_plugins(camera::CameraPlugin);
        app.add_plugins(physics::PhysicsPlugin);

        // attach application's internal system set to bevy's Update
        app.configure_sets(
            Update,
            (
                ApplicationSystemSet::TickTimers,
                ApplicationSystemSet::RecordInput,
                ApplicationSystemSet::Update,
            )
                .chain(),
        );

        #[cfg(feature = "development")]
        app.add_plugins(development::DevelopmentPlugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// Make sure to order it in the `configure_sets` call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum ApplicationSystemSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
