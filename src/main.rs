mod buffered_inputs;
mod constants;
mod player;
mod rendering;
mod ruler;

use bevy::prelude::*;
use {
    bevy::window::WindowResolution,
    buffered_inputs::update_buffered_inputs,
    constants::RESOLUTION,
    player::{spawn_player, update_player},
    rendering::{fit_canvas, move_camera, setup_camera},
    ruler::{spawn_ruler_markings, update_ruler_markings},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(960.0, 640.0),
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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, spawn_player, spawn_ruler_markings))
        .add_systems(
            Update,
            (
                fit_canvas,
                (move_camera, update_ruler_markings).chain(),
                (update_buffered_inputs, update_player).chain(),
            ),
        )
        .run();
}
