use bevy::prelude::*;

use crate::constants::{
    LEFT_KEY_CODES, PLAYER_HORIZONTAL_DRAG, PLAYER_HORIZONTAL_SPEED, RIGHT_KEY_CODES,
};

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

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut player_transform) = match player_query.get_single_mut() {
        Ok(player) => player,
        Err(_) => return,
    };

    let current_horizontal_velocity = player.velocity.x;

    for key_code in LEFT_KEY_CODES {
        if input.pressed(key_code) {
            player.velocity.x = -PLAYER_HORIZONTAL_SPEED;
            break;
        }
    }

    for key_code in RIGHT_KEY_CODES {
        if input.pressed(key_code) {
            player.velocity.x = if current_horizontal_velocity != player.velocity.x {
                0.0
            } else {
                PLAYER_HORIZONTAL_SPEED
            };
            break;
        }
    }

    if player.velocity.x == current_horizontal_velocity {
        player.velocity.x = player.velocity.x.lerp(
            0.0,
            PLAYER_HORIZONTAL_DRAG.powf(100.0 * time.delta_seconds()),
        );
    }

    let x_dir = player.velocity.x;

    player.velocity.x = if x_dir.abs() > PLAYER_HORIZONTAL_SPEED {
        PLAYER_HORIZONTAL_SPEED * (x_dir / x_dir.abs())
    } else {
        player.velocity.x
    };

    player_transform.translation.x += player.velocity.x * time.delta_seconds();
    // player_transform.translation.y += player.velocity.y * time.delta_seconds();
}
