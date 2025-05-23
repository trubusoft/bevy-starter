use avian2d::prelude::PhysicsPlugins as AvianPhysicsPlugins;
use bevy::prelude::{App, Plugin};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AvianPhysicsPlugins::default().with_length_unit(20.0));
    }
}
