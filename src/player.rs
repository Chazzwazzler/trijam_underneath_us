use bevy::prelude::*;

use {
    crate::{
        buffered_inputs::{self, BufferedInput},
        constants::{
            ACTION_KEY_CODES, DIVE_TIME, LEFT_KEY_CODES, PLAYER_FALL_SPEED,
            PLAYER_HORIZONTAL_MOVE_DRAG, PLAYER_HORIZONTAL_MOVE_SPEED, PLAYER_JUMP_SPEED,
            PLAYER_VERTICAL_SPEED_MAX, RIGHT_KEY_CODES,
        },
    },
    std::time::Duration,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    player: Player,
}

#[derive(PartialEq)]
enum PlayerAnimation {
    None,
    Falling,
    Flapping,
    Diving,
}

impl PlayerAnimation {
    pub fn get_index(self) -> usize {
        return self as usize * 7 as usize;
    }
}

#[derive(Component)]
pub struct Player {
    health: i32,
    velocity: Vec2,
    current_animation: PlayerAnimation,
    animation_timer: Timer,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(PlayerBundle {
        sprite_sheet_bundle: SpriteSheetBundle {
            texture: asset_server.load("player_spritesheet.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::new(16.0, 16.0),
                    7,
                    4,
                    None,
                    None,
                )),
                ..default()
            },
            ..default()
        },
        player: Player {
            health: 100,
            velocity: Vec2::ZERO,
            current_animation: PlayerAnimation::Falling,
            animation_timer: Timer::new(Duration::from_millis(80), TimerMode::Repeating),
        },
    });
}

pub fn update_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform, &mut TextureAtlas, &mut Sprite)>,
    buffered_inputs: Query<&BufferedInput>,
) {
    let (mut player, mut player_transform, mut player_spritesheet, mut player_sprite) =
        match player_query.get_single_mut() {
            Ok(player) => player,
            Err(_) => return,
        };

    update_player_animation(&time, &mut player, &mut player_spritesheet);
    update_player_horizontal_velocity(&time, &input, &mut player, &mut player_sprite);
    update_player_vertical_velocity(
        &time,
        &input,
        &mut commands,
        &mut player,
        &mut player_spritesheet,
    );
    update_player_diving(buffered_inputs, &mut player, &mut player_spritesheet);

    player_transform.translation.x += player.velocity.x * time.delta_seconds();
    player_transform.translation.y += player.velocity.y * time.delta_seconds();
}

pub fn update_player_diving(
    buffered_inputs: Query<&BufferedInput>,
    player: &mut Player,
    player_spritesheet: &mut TextureAtlas,
) {
    let mut elapsed_dive_secs: f32 = f32::MAX;
    let mut dive: bool = player.current_animation == PlayerAnimation::Diving;

    for buffered_input in buffered_inputs.iter() {
        if buffered_input.timer.elapsed_secs() < elapsed_dive_secs {
            elapsed_dive_secs = buffered_input.timer.elapsed_secs();
            dive = buffered_input.pressed;
        }
    }

    dive = elapsed_dive_secs > DIVE_TIME && dive;

    if player.current_animation == PlayerAnimation::Diving {
        if !dive {
            player.current_animation = PlayerAnimation::Falling;
            player_spritesheet.index = PlayerAnimation::Falling.get_index();
        }

        return;
    }

    if player.current_animation != PlayerAnimation::Diving && dive {
        player.current_animation = PlayerAnimation::Diving;
        player_spritesheet.index = PlayerAnimation::Diving.get_index();
    }
}

pub fn update_player_animation(
    time: &Res<Time>,
    player: &mut Player,
    player_spritesheet: &mut TextureAtlas,
) {
    player.animation_timer.tick(time.delta());

    if player.animation_timer.just_finished() {
        match player.current_animation {
            PlayerAnimation::None => (),
            PlayerAnimation::Falling => {
                player_spritesheet.index =
                    ((player_spritesheet.index % 7 + 1) % 3) + PlayerAnimation::Falling.get_index()
            }
            PlayerAnimation::Flapping => {
                player_spritesheet.index = ((player_spritesheet.index % 7 + 1) % 4)
                    + PlayerAnimation::Flapping.get_index();

                if ((player_spritesheet.index % 7) + 1) % 4 == 0 {
                    player.current_animation = PlayerAnimation::Falling;
                }
            }
            PlayerAnimation::Diving => {
                if (player_spritesheet.index % 7 + 1) % 7 != 0 {
                    player_spritesheet.index = ((player_spritesheet.index % 7 + 1) % 7)
                        + PlayerAnimation::Diving.get_index();
                }
            }
        }
    }
}

pub fn update_player_horizontal_velocity(
    time: &Res<Time>,
    input: &Res<ButtonInput<KeyCode>>,
    player: &mut Player,
    player_sprite: &mut Sprite,
) {
    let current_horizontal_velocity = player.velocity.x;

    for key_code in LEFT_KEY_CODES {
        if input.pressed(key_code) {
            player.velocity.x = -PLAYER_HORIZONTAL_MOVE_SPEED;
            player_sprite.flip_x = true;
            break;
        }
    }

    for key_code in RIGHT_KEY_CODES {
        if input.pressed(key_code) {
            player.velocity.x = if current_horizontal_velocity != player.velocity.x {
                0.0
            } else {
                player_sprite.flip_x = false;
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
    player_spritesheet: &mut TextureAtlas,
) {
    let current_vertical_velocity = player.velocity.y;

    for key_code in ACTION_KEY_CODES {
        if input.just_pressed(key_code) {
            player.velocity.y = PLAYER_JUMP_SPEED;
            commands.spawn(BufferedInput {
                timer: Timer::new(Duration::from_millis(20000), TimerMode::Once),
                pressed: true,
            });
            player.current_animation = PlayerAnimation::Flapping;
            player_spritesheet.index = PlayerAnimation::Flapping.get_index();
            break;
        }
    }

    for key_code in ACTION_KEY_CODES {
        if input.just_released(key_code) {
            commands.spawn(BufferedInput {
                timer: Timer::new(Duration::from_millis(20000), TimerMode::Once),
                pressed: false,
            });
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
