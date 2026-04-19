/// This file has the definitions for the structs containing the textures

// NOTE: Dragondude! Do you think we should have seperate files for each of the structs or is this fine?
use macroquad::prelude::*;

pub struct Level {
    pub base_tile: Texture2D
    // We'll add in more easily as we go
}
impl Level {
    pub async fn load() -> Self {
        // EXAMPLE
            let base_tile = load_texture("assets/textures/tiles/base_tile.png").await.expect("Failed to load assets/textures/tiles/base_tile.png");
            fighter.set_filter(FilterMode::Nearest); 

            Self {
                base_tile,
            }
    }
}

pub struct Enemies {
    pub turtle_master: Texture2D,
}
impl Enemies {
    pub async fn load() -> Self {
            Self {

            }
    }
}

pub struct Races {
    pub void_crawler: Texture2D,
    pub human: Texture2D,
}
impl Buildings {
    pub async fn load() -> Self {
            Self {

            }
    }
}
