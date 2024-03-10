mod buffered_inputs;
mod constants;
mod player;
mod rendering;

use bevy::prelude::*;
use {
    buffered_inputs::update_buffered_inputs,
    constants::RESOLUTION,
    player::spawn_player,
    rendering::{fit_canvas, setup_camera},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resize_constraints: WindowResizeConstraints {
                            min_width: RESOLUTION.width as f32,
                            min_height: RESOLUTION.height as f32,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, (fit_canvas, update_buffered_inputs))
        .run();
}
