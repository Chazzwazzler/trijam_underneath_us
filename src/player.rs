use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    player: Player,
}

#[derive(Component)]
pub struct Player {
    health: i32,
    velocity: Vec2,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        sprite_sheet_bundle: SpriteSheetBundle {
            texture: asset_server.load("player_placeholder.png"),
            ..default()
        },
        player: Player {
            health: 100,
            velocity: Vec2::ZERO,
        },
    });
}
