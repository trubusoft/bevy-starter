use bevy::prelude::{App, DefaultPlugins};
use bevy_starter::example::PeoplePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}
