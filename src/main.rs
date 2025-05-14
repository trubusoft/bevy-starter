use bevy::prelude::App;
use bevy::DefaultPlugins;

use people::plugins::PeoplePlugin;

mod people;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}
