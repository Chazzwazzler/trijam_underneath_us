use crate::constants::{HIGH_RES_LAYER, RESOLUTION};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::window::WindowResized;

#[derive(Component)]
pub struct InGameCamera;

#[derive(Component)]
pub struct RenderCamera;

#[derive(Component)]
struct Canvas;

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: RESOLUTION,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    canvas.resize(RESOLUTION);

    let image_handle = images.add(canvas);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                clear_color: ClearColorConfig::Custom(Color::rgb(0.5, 0.5, 0.6)),
                ..default()
            },
            ..default()
        },
        InGameCamera,
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_LAYER,
    ));

    commands.spawn((Camera2dBundle::default(), RenderCamera, HIGH_RES_LAYER));
}

pub fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<RenderCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RESOLUTION.width as f32;
        let v_scale = event.height / RESOLUTION.height as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).floor();
    }
}
