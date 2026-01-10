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

#[macroquad::main(window_conf)]
async fn main() {
    let texture: Texture2D = load_texture("assets/slug_16x16.png").await.unwrap();

    texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);

        let data_size = vec2(64.0, 64.0);

        let draw_pos = screen_center() - data_size * 0.5;

        draw_texture_ex(
            &texture,
            draw_pos.x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    0.0,  // x
                    0.0,  // y
                    16.0, // width
                    16.0, // height
                )),
                dest_size: Some(data_size),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
