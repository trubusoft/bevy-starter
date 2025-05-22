use bevy::prelude::SystemSet;

/// High-level groupings of systems for the app in the `Update` schedule.
/// Make sure to order it in the `configure_sets` call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ApplicationSchedule {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
