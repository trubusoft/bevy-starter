use bevy::prelude::{App, IntoScheduleConfigs, Plugin, SystemSet, Update};

/// High-level groupings of systems for the app in the `Update` schedule.
/// Make sure to order it in the `configure_sets` call below.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ApplicationSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        // attach application's internal system set to bevy's Update
        app.configure_sets(
            Update,
            (
                ApplicationSystems::TickTimers,
                ApplicationSystems::RecordInput,
                ApplicationSystems::Update,
            )
                .chain(),
        );
    }
}
