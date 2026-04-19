/// This file loads and exposes all assets to the rest of the codebase
pub mod definitions;
use definitions::*;

mod constants;
pub use constants::*;

pub struct Assets {
    pub level: Level,
    pub enemies: Enemies,
    pub player_races: Races,
}

impl Assets {
    pub async fn load() -> Self {
        let level = Level::load().await;
        let enemies = Enemies::load().await;
        let player_races = Races::load().await;

        Self {
            level,
            enemies,
            player_races,
        }
    }
}


