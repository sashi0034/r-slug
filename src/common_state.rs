use crate::field::Field;
use crate::game_params::{screen_center, SCREEN_HEIGHT, SCREEN_SCALE, SCREEN_WIDTH};
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

pub struct FieldData {
    pub tiles_x: i32,
    pub tiles_y: i32,
    pub rect: Rect,
}

pub struct CommonState {
    pub time: f32,
    pub field_data: FieldData,
}

#[inline]
pub fn field_tile_size() -> Vec2 {
    vec2(24.0, 24.0)
}

fn filed_date() -> FieldData {
    let tiles_x = 10;
    let tiles_y = 5;

    let mut rect = Rect::new(
        0.0,
        0.0,
        tiles_x as f32 * field_tile_size().x,
        tiles_y as f32 * field_tile_size().y,
    );
    rect.move_to(vec2(
        screen_center().x - rect.w * 0.5,
        screen_center().y - rect.h * 0.5,
    ));

    FieldData {
        tiles_x,
        tiles_y,
        rect,
    }
}

impl CommonState {
    pub fn new() -> CommonState {
        CommonState {
            time: 0.0,
            field_data: filed_date(),
        }
    }
}
