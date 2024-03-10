use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("player_placeholder.png"),
        ..Default::default()
    });
}
