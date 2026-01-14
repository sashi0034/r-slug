use crate::animation_frame;
use crate::common_state::{field_tile_size, FieldData};
use crate::game_params::screen_center;
use crate::game_resources::game_resources;
use crate::types::Horizontal2;
use macroquad::prelude::*;

pub struct Field {
    num_columns: i32,
    num_rows: i32,
}

pub fn draw_field(field: &FieldData) {
    for ty in 0..field.tiles_y {
        for tx in 0..field.tiles_x {
            let source_size = field_tile_size();

            let x = field.rect.x + tx as f32 * source_size.x;
            let y = field.rect.y + ty as f32 * source_size.y;
            draw_texture_ex(
                &game_resources().textures.floor,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        0.0,           // x
                        0.0,           // y
                        source_size.x, // width
                        source_size.y, // height
                    )),
                    dest_size: Some(source_size),
                    ..Default::default()
                },
            );
        }
    }
}
