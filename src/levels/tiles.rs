// This file holds some level abstractions that are used for managing levels 

// Describes position in the map grid
pub struct Position {
    // We are not going to have to deal with negative positions and u8 is a tiny bit too small.
    x: u16,
    y: u16,
}
pub struct Tile {
    position: Position,
    // tile_type is used to be able to match the correct texture to the correct tile.
    // One type = One texture
    // This is good since we do not want tiles to own their own textures since that increases
    // complexity, we want all textures to be managed in one place and handed out when needed.
    tile_type: TileType,
}

pub enum TileType {
    Grass,
    Floor,
    // Yeah I have no idea what tiles there will be, just trust the process
}
