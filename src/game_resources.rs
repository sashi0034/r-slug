use macroquad::prelude::*;
use std::sync::{Mutex, OnceLock};

#[derive(Debug)]
pub struct GameTextures {
    pub player: Texture2D,
    pub floor: Texture2D,
}

async fn textures() -> GameTextures {
    let player: Texture2D = macroquad::prelude::load_texture("assets/slug_16x16.png")
        .await
        .unwrap();
    player.set_filter(macroquad::prelude::FilterMode::Nearest);

    let floor: Texture2D = macroquad::prelude::load_texture("assets/floor_24x24.png")
        .await
        .unwrap();
    floor.set_filter(macroquad::prelude::FilterMode::Nearest);

    GameTextures { player, floor }
}

#[derive(Debug)]
pub struct GameResources {
    pub textures: GameTextures,
}

static GAME_RESOURCES: OnceLock<GameResources> = OnceLock::new();

pub async fn init_game_resources() {
    let resources = GameResources {
        textures: textures().await,
    };

    GAME_RESOURCES
        .set(resources)
        .expect("GameResources already initialized");
}

pub fn game_resources() -> &'static GameResources {
    GAME_RESOURCES.get().expect("GameResources not initialized")
}
