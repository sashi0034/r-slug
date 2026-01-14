use macroquad::prelude::{vec2, Vec2};

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

#[inline]
pub fn screen_size() -> Vec2 {
    vec2(SCREEN_WIDTH, SCREEN_HEIGHT)
}

#[inline]
pub fn screen_center() -> Vec2 {
    screen_size() * 0.5
}

pub const SCREEN_SCALE: f32 = 4.0;
