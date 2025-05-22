use bevy::{
    input::common_conditions::input_just_pressed,
    log::debug,
    prelude::{
        App, ButtonInput, Camera2d, Commands, Component, IntoScheduleConfigs, KeyCode, Plugin,
        Query, Res, ResMut, Resource, Startup, Transform, Update, With,
    },
};

const TOGGLE_KEY: KeyCode = KeyCode::Digit1;
const MOVE_SPEED: f32 = 5.0;

#[derive(Resource, Default)]
pub struct CameraOptions {
    pub moveable: bool,
    pub zoomable: bool,
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
