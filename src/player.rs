use crate::animation_frame;
use crate::common_state::CommonState;
use crate::game_params::screen_center;
use crate::game_resources::game_resources;
use crate::types::Horizontal2;
use macroquad::prelude::*;

pub struct Player {
    pos: Vec2,
    dir: Horizontal2,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: screen_center(),
            dir: Horizontal2::Left,
        }
    }

    pub fn update(&mut self, common_state: &CommonState) {
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

        self.dir = if move_dir.x > 0.0 {
            Horizontal2::Right
        } else if move_dir.x < 0.0 {
            Horizontal2::Left
        } else {
            self.dir
        };

        // -----------------------------------------------

        let source_size = vec2(16.0, 16.0);
        let draw_pos = self.pos - source_size * 0.5;

        draw_texture_ex(
            &game_resources().textures.player,
            draw_pos.x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    16.0 * animation_frame(common_state.time, 4, 250) as f32, // x
                    0.0,                                                      // y
                    source_size.x,                                            // width
                    source_size.y,                                            // height
                )),
                dest_size: Some(source_size),
                flip_x: self.dir == Horizontal2::Right,
                ..Default::default()
            },
        );
    }
}
