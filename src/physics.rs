use avian2d::prelude::PhysicsPlugins as AvianPhysicsPlugins;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct PhysicsPlugins;

impl Plugin for PhysicsPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(AvianPhysicsPlugins::default().with_length_unit(20.0));
    }
}
