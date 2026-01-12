mod game_params;
mod game_resources;
mod player;

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

    let mut player = Player::new();

    loop {
        clear_background(BLACK);

        player.update();

        next_frame().await;
    }
}
