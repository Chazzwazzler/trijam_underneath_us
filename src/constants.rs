use bevy::{
    input::keyboard::KeyCode,
    render::{render_resource::Extent3d, view::RenderLayers},
};

pub const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(1);
pub const RESOLUTION: Extent3d = Extent3d {
    width: 180,
    height: 320,
    depth_or_array_layers: 1,
};

pub const PLAYER_HORIZONTAL_MOVE_SPEED: f32 = 170.0;
pub const PLAYER_HORIZONTAL_MOVE_DRAG: f32 = 0.4;
pub const PLAYER_VERTICAL_SPEED_MAX: f32 = 180.0;
pub const PLAYER_FALL_SPEED: f32 = 1100.0;
pub const PLAYER_JUMP_SPEED: f32 = 2800.0;

pub const LEFT_KEY_CODES: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
pub const RIGHT_KEY_CODES: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
pub const ACTION_KEY_CODES: [KeyCode; 2] = [KeyCode::ShiftRight, KeyCode::Space];
