use std::time::Duration;

pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;
pub const TARGET_FPS: u64 = 60;
pub const FOV: f32 = 70.0;
pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / TARGET_FPS); // ~16 ms
pub const NEAR_CLIP_PLANE: f32 = -0.1; // z axis
pub const CAMERA_SPEED: f32 = 4.0;
pub const MOUSE_SENSITIVITY: f32 = 100.0;
