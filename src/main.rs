mod common_state;
mod field;
mod game_params;
mod game_resources;
mod player;
mod types;

use crate::field::draw_field;
use crate::game_params::{screen_size, SCREEN_SCALE};
use crate::game_resources::init_game_resources;
use crate::player::Player;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "My Rust Game".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

fn animation_frame(time: f32, frame_count: i32, duration_ms: i32) -> i32 {
    ((time * 1000.0).floor() as i32 / duration_ms) % frame_count
}

#[macroquad::main(window_conf)]
async fn main() {
    init_game_resources().await;

    let mut common_state = common_state::CommonState::new();

    let mut player = Player::new();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        common_state.time += get_frame_time();

        {
            set_camera(&Camera2D {
                zoom: SCREEN_SCALE * 2.0 / screen_size(),
                target: screen_size() * 0.5,
                ..Default::default()
            });

            draw_field(&common_state.field_data);

            player.update(&common_state);
        }

        next_frame().await;
    }
}
