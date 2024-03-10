use bevy::prelude::*;
use {
    crate::{constants::RESOLUTION, rendering::InGameCamera},
    bevy::sprite::Anchor,
};

#[derive(Component)]
pub struct RulerMarking;

pub fn spawn_ruler_markings(mut commands: Commands, asset_server: Res<AssetServer>) {
    let count = 3;

    for i in 0..count {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("- {}", i * 5),
                    TextStyle {
                        font: asset_server.load("fonts/primary.ttf"),
                        font_size: 8.0,
                        ..default()
                    },
                ),
                text_anchor: Anchor::TopLeft,
                transform: Transform::from_xyz(
                    -(RESOLUTION.width as f32) / 2.0 + 2.0,
                    i as f32 * -(RESOLUTION.height as f32 * 1.5 / count as f32),
                    1.0,
                ),
                ..default()
            },
            RulerMarking,
        ));
    }
}

pub fn update_ruler_markings(
    camera_query: Query<&Transform, (With<InGameCamera>, Without<RulerMarking>)>,
    mut ruler_markings: Query<(&mut Transform, &mut Text), With<RulerMarking>>,
) {
    let camera_transform = match camera_query.get_single() {
        Ok(camera_transform) => camera_transform,
        Err(_) => return,
    };

    for (mut ruler_marking_transform, mut ruler_marking_text) in ruler_markings.iter_mut() {
        if ruler_marking_transform.translation.y
            > camera_transform.translation.y + RESOLUTION.height as f32 / 2.0
        {
            ruler_marking_transform.translation.y -= RESOLUTION.height as f32 * 1.5;
            for section in ruler_marking_text.sections.iter_mut() {
                section.value = format!(
                    "- {}",
                    (ruler_marking_transform.translation.y / (RESOLUTION.height as f32 / 2.0)
                        * 5.0)
                        .round()
                        .abs()
                )
            }
        }
    }
}
