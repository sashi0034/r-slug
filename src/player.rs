use crate::animation_frame;
use macroquad::color::WHITE;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::prelude::{draw_texture_ex, get_frame_time, DrawTextureParams, Texture2D};
use crate::game_params::screen_center;

pub struct Player {
    texture: Texture2D,
    time: f32,
    pos: Vec2,
}

impl Player {
    pub async fn new() -> Player {
        let texture: Texture2D = macroquad::prelude::load_texture("assets/slug_16x16.png")
            .await
            .unwrap();
        texture.set_filter(macroquad::prelude::FilterMode::Nearest);
        Player {
            texture,
            time: 0.0,
            pos: screen_center(),
        }
    }

    pub fn update(&mut self) {
        self.time += get_frame_time();

        let data_size = vec2(64.0, 64.0);

        let mut move_dir = vec2(0.0, 0.0);
        if is_key_down(KeyCode::W) {
            move_dir.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }

        let speed = 100.0;
        self.pos += move_dir.normalize_or_zero() * speed * get_frame_time();

        // -----------------------------------------------

        let draw_pos = self.pos - data_size * 0.5;

        draw_texture_ex(
            &self.texture,
            draw_pos.x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    16.0 * animation_frame(self.time, 4, 250) as f32, // x
                    0.0,                                              // y
                    16.0,                                             // width
                    16.0,                                             // height
                )),
                dest_size: Some(data_size),
                ..Default::default()
            },
        );
    }
}
