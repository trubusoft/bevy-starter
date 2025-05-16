use bevy::prelude::{default, Window, WindowPlugin};
use std::num::NonZero;

/// Get the preferred [Window], which can be customized.
///
/// Example:
///
/// ```
/// # use bevy::prelude::{Window, default};
/// let primary_window = Window {
///     // custom title
///     title: String::from("Bevy Starter"),
///
///     // custom resolution
///     resolution: (800., 600.).into(),
///
///     ..default()
/// };
/// ```
pub fn window_plugin() -> WindowPlugin {
    let primary_window = Window {
        title: String::from("Bevy Starter"),
        resizable: false,
        canvas: Some(String::from("#bevy")),
        desired_maximum_frame_latency: NonZero::new(1u32),
        ..default()
    };

    WindowPlugin {
        primary_window: Some(primary_window),
        ..default()
    }
}
