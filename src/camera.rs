use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::{Projection, Single};
use bevy::{
    input::common_conditions::input_just_pressed,
    log::debug,
    prelude::{
        App, ButtonInput, Camera2d, Commands, Component, IntoScheduleConfigs, KeyCode, Plugin,
        Query, Res, ResMut, Resource, Startup, Transform, Update, With,
    },
};
use std::ops::Range;

const TOGGLE_KEY: KeyCode = KeyCode::Digit1;
const MOVE_SPEED: f32 = 5.0;
const ZOOM_RANGE: Range<f32> = 0.1..10.0;
const ZOOM_SPEED: f32 = 0.2;

#[derive(Resource)]
pub struct CameraOptions {
    pub moveable: bool,
    pub zoomable: bool,
    /// Clamp the orthographic camera's scale to this range
    pub zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub zoom_speed: f32,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            moveable: false,
            zoomable: false,
            zoom_range: ZOOM_RANGE,
            zoom_speed: ZOOM_SPEED,
        }
    }
}

impl CameraOptions {
    pub fn toggle_moveable(&mut self) {
        self.moveable = !self.moveable;
    }

    pub fn toggle_zoomable(&mut self) {
        self.zoomable = !self.zoomable;
    }
}

/// Marker component for main camera
#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraOptions>();
        app.add_systems(Startup, initialize_main_camera);
        app.add_systems(
            Update,
            toggle_moveable_option.run_if(input_just_pressed(TOGGLE_KEY)),
        );
        app.add_systems(
            Update,
            toggle_zoomable_option.run_if(input_just_pressed(TOGGLE_KEY)),
        );
        app.add_systems(Update, move_camera);
        app.add_systems(Update, zoom_camera);
    }
}

fn initialize_main_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn toggle_moveable_option(mut options: ResMut<CameraOptions>) {
    options.toggle_moveable();
    debug!(
        "toggling CameraOptions.moveable to: {:?}",
        &options.moveable
    );
}

fn toggle_zoomable_option(mut options: ResMut<CameraOptions>) {
    options.toggle_zoomable();
    debug!(
        "toggling CameraOptions.zoomable to: {:?}",
        &options.zoomable
    );
}

fn move_camera(
    option: Res<CameraOptions>,
    input: Res<ButtonInput<KeyCode>>,
    mut camera_transforms: Query<&mut Transform, With<MainCamera>>,
) {
    if option.moveable {
        let any_pressed = input.pressed(KeyCode::KeyA)
            | input.pressed(KeyCode::KeyD)
            | input.pressed(KeyCode::KeyW)
            | input.pressed(KeyCode::KeyS);

        if any_pressed {
            if let Ok(mut camera_transform) = camera_transforms.single_mut() {
                if input.pressed(KeyCode::KeyA) {
                    camera_transform.translation.x -= MOVE_SPEED;
                }
                if input.pressed(KeyCode::KeyD) {
                    camera_transform.translation.x += MOVE_SPEED;
                }
                if input.pressed(KeyCode::KeyW) {
                    camera_transform.translation.y += MOVE_SPEED;
                }
                if input.pressed(KeyCode::KeyS) {
                    camera_transform.translation.y -= MOVE_SPEED;
                }
            }
        }
    }
}

/// Mainly referenced from https://bevyengine.org/examples/camera/projection-zoom/
fn zoom_camera(
    option: Res<CameraOptions>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
    camera: Single<&mut Projection, With<MainCamera>>,
) {
    if option.zoomable {
        if let Projection::Orthographic(ref mut orthographic) = *camera.into_inner() {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * option.zoom_speed;

            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom)
                .clamp(option.zoom_range.start, option.zoom_range.end);
        }
    }
}
