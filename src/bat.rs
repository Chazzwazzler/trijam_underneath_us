use std::time::Duration;

use crate::constants::RESOLUTION;
use crate::player::Player;
use crate::rendering::InGameCamera;
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct Spawner {
    pub time_left: Timer,
}

#[derive(Component)]
pub struct BatTime {
    pub anim_timer: Timer,
}

const BAT_SPEED: f32 = 64.0;
const BAT_SPAWN_MIN: f32 = 2.0;
const BAT_SPAWN_MAX: f32 = 5.0;

pub fn init_bat_spawner(mut spawner: ResMut<Spawner>) {
    let mut rng = rand::thread_rng();
    spawner.time_left = Timer::new(
        Duration::from_secs(rng.gen_range(BAT_SPAWN_MIN..BAT_SPAWN_MAX) as u64),
        TimerMode::Once,
    );
}

pub fn spawn_bats(
    time: Res<Time>,
    mut spawner: ResMut<Spawner>,
    mut camera_query: Query<&mut Transform, With<InGameCamera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let camera_transform = match camera_query.get_single_mut() {
        Ok(camera) => camera,
        Err(_) => return,
    };
    spawner.time_left.tick(time.delta());

    if spawner.time_left.just_finished() {
        let mut rng = rand::thread_rng();
        spawner.time_left = Timer::new(
            Duration::from_secs(rng.gen_range(BAT_SPAWN_MIN..BAT_SPAWN_MAX) as u64),
            TimerMode::Once,
        );
        commands
            .spawn(SpriteSheetBundle {
                texture: asset_server.load("bat.png"),
                atlas: TextureAtlas {
                    layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        Vec2::new(16.0, 16.0),
                        4,
                        1,
                        None,
                        None,
                    )),
                    ..default()
                },
                transform: Transform::from_xyz(
                    rng.gen_range(
                        -1.0 * (RESOLUTION.height as f32 / 2.0)..(RESOLUTION.height as f32 / 2.0),
                    ),
                    camera_transform.translation.y - (RESOLUTION.height as f32 / 2.0),
                    1.0,
                ),
                ..default()
            })
            .insert(BatTime {
                anim_timer: Timer::new(Duration::from_millis(80), TimerMode::Repeating),
            });
    }
}

pub fn update_bats(
    time: Res<Time>,
    mut bats: Query<(&mut BatTime, &mut TextureAtlas, &mut Transform)>,
    player: Query<&Transform, (With<Player>, Without<BatTime>)>,
) {
    for (mut bat_thing, mut bat_sprites, mut bat_trans) in bats.iter_mut() {
        bat_thing.anim_timer.tick(time.delta());
        if bat_thing.anim_timer.just_finished() {
            if bat_sprites.index == 3 {
                bat_sprites.index = 0;
            } else {
                bat_sprites.index += 1;
            }
        }
        let dir =
            (player.get_single().unwrap().translation - bat_trans.translation).normalize_or_zero();
        bat_trans.translation += dir * BAT_SPEED * time.delta().as_secs_f32();
    }
}
