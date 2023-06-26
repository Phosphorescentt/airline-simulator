pub mod airline_plugin;

use bevy::prelude::*;

use airline_plugin::AirlinePlugin;

fn main() {
    App::new()
        // Set camera background colour
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AirlinePlugin)
        .run();
}
