use bevy::prelude::*;
mod startup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(startup::StartupPlugin)
        .run();
}

