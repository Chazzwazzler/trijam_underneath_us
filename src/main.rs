mod constants;
mod player;
mod rendering;

use bevy::prelude::*;
use {
    player::spawn_player,
    rendering::{fit_canvas, setup_camera},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, fit_canvas)
        .run();
}
