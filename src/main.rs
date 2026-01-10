mod game_params;

use crate::game_params::screen_center;
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

    let mut time: f32 = 0.0;

    let mut pos: Vec2 = screen_center();

    loop {
        clear_background(BLACK);

        time += get_frame_time();

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
        pos += move_dir.normalize_or_zero() * speed * get_frame_time();

        let draw_pos = pos - data_size * 0.5;

        draw_texture_ex(
            &texture,
            draw_pos.x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    16.0 * animation_frame(time, 4, 250) as f32, // x
                    0.0,                                         // y
                    16.0,                                        // width
                    16.0,                                        // height
                )),
                dest_size: Some(data_size),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
