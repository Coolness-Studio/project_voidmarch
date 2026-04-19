/// This file handles all level logic

// Not yet implemented, will do the level loading logic.
// When implemented it should have some function that returns a Vec<Tile> or similar (note that
// Tile is not implemented) that can then be iterated through and drawn. Another function called
// something like draw_tiles or similar could be used to make the draw_level function less
// cluttered.
mod read_level;
use read_level::*;

pub fn draw_level(id: u8) {
    match id {
        // Level 0 might be tutorial or smth
        0u8 => ,
        1u8 => ,
        2u8 => ,
        3u8 => ,
    }
}
