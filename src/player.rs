use bevy::prelude::*;

use {
    crate::{
        buffered_inputs::BufferedInput,
        constants::{
            ACTION_KEY_CODES, LEFT_KEY_CODES, PLAYER_FALL_SPEED, PLAYER_HORIZONTAL_MOVE_DRAG,
            PLAYER_HORIZONTAL_MOVE_SPEED, PLAYER_JUMP_SPEED, PLAYER_VERTICAL_SPEED_MAX,
            RIGHT_KEY_CODES,
        },
    },
    std::time::Duration,
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

    update_player_horizontal_velocity(&time, &input, &mut player);
    update_player_vertical_velocity(&time, &input, &mut commands, &mut player);

    player_transform.translation.x += player.velocity.x * time.delta_seconds();
    player_transform.translation.y += player.velocity.y * time.delta_seconds();
}

pub fn update_player_horizontal_velocity(
    time: &Res<Time>,
    input: &Res<ButtonInput<KeyCode>>,
    player: &mut Player,
) {
    let current_horizontal_velocity = player.velocity.x;

    for key_code in LEFT_KEY_CODES {
        if input.pressed(key_code) {
            player.velocity.x = -PLAYER_HORIZONTAL_MOVE_SPEED;
            break;
        }
    }

    for key_code in RIGHT_KEY_CODES {
        if input.pressed(key_code) {
            player.velocity.x = if current_horizontal_velocity != player.velocity.x {
                0.0
            } else {
                PLAYER_HORIZONTAL_MOVE_SPEED
            };
            break;
        }
    }

    if player.velocity.x == current_horizontal_velocity {
        player.velocity.x = player.velocity.x.lerp(
            0.0,
            PLAYER_HORIZONTAL_MOVE_DRAG.powf(100.0 * time.delta_seconds()),
        );
    }

    let x_dir = player.velocity.x;

    player.velocity.x = if x_dir.abs() > PLAYER_HORIZONTAL_MOVE_SPEED {
        PLAYER_HORIZONTAL_MOVE_SPEED * (x_dir / x_dir.abs())
    } else {
        player.velocity.x
    };
}

pub fn update_player_vertical_velocity(
    time: &Res<Time>,
    input: &Res<ButtonInput<KeyCode>>,
    commands: &mut Commands,
    player: &mut Player,
) {
    let current_vertical_velocity = player.velocity.y;

    for key_code in ACTION_KEY_CODES {
        if input.just_pressed(key_code) {
            player.velocity.y = PLAYER_JUMP_SPEED;
            commands.spawn(BufferedInput {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Once),
            });
            break;
        }
    }

    if player.velocity.y == current_vertical_velocity {
        player.velocity.y -= PLAYER_FALL_SPEED * time.delta_seconds();
    }

    let y_dir = player.velocity.y;

    player.velocity.y = if y_dir.abs() > PLAYER_VERTICAL_SPEED_MAX {
        PLAYER_VERTICAL_SPEED_MAX * (y_dir / y_dir.abs())
    } else {
        player.velocity.y
    };
}
