use bevy::render::{render_resource::Extent3d, view::RenderLayers};

pub const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(1);
pub const RESOLUTION: Extent3d = Extent3d {
    width: 144,
    height: 256,
    depth_or_array_layers: 1,
};

pub const PLAYER_TOP_HORIZONTAL_SPEED: f32 = 10.0;
pub const PLAYER_TOP_VERTICAL_SPEED: f32 = 10.0;
