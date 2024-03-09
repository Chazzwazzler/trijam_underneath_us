use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::WindowResolution,
};

const WINDOW_SIZE: (f32, f32) = (640.0, 640.0);
const TILE_SIZE: (u32, u32) = (16, 16);
const TILE_NUM: (u32, u32) = (20, 20);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, spawn_player))
        .run();
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("player_placeholder.png"),
        ..Default::default()
    });
}

fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut windows: Query<&mut Window>,
) {
    for mut window in windows.iter_mut() {
        window.resolution = WindowResolution::new(WINDOW_SIZE.0, WINDOW_SIZE.1)
            .with_scale_factor_override(WINDOW_SIZE.0 / (TILE_NUM.0 * TILE_SIZE.0) as f32);
    }
    let canvas_size = Extent3d {
        width: TILE_NUM.0 * TILE_SIZE.0,
        height: TILE_NUM.1 * TILE_SIZE.1,
        ..default()
    };

    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
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
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);
    commands.spawn((Camera2dBundle {
        camera: Camera {
            order: -1,
            target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        ..default()
    },));

    commands.spawn((SpriteBundle {
        texture: image_handle,
        ..default()
    },));
    commands.spawn(Camera2dBundle::default());
}
