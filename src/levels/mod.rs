/// This file handles all level logic
mod read_level;
use read_level::*;

mod tiles;
pub use tiles::*;

pub fn draw_level(id: u8) {
    match id {
        // Level 0 might be tutorial or smth
        0 => todo!("tutorial logic"),
        i => todo!("level {} logic", i),
    }
}

