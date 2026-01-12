mod game_params;
mod player;

use crate::game_params::screen_center;
use crate::player::Player;
use macroquad::miniquad::window::high_dpi;
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
    let texture: Texture2D = load_texture("assets/slug_16x16.png").await.unwrap();

    texture.set_filter(FilterMode::Nearest);

    let mut player = Player::new().await;

    loop {
        clear_background(BLACK);

        player.update();

        next_frame().await;
    }
}
