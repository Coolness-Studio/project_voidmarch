/// This file handles all level logic
use std::{error::Error, fmt};

use macroquad::prelude::*;
use tellus_level::{LayerKind, Level, LevelFormatError, LevelIoError};

mod tiles;

use super::assets::*;

#[derive(Debug)]
pub enum LevelError {
    UnknownLevel { id: u8 },
    LoadFailed {
        id: u8,
        path: String,
        source: LevelIoError,
    },
    TileReadFailed {
        id: u8,
        x: u16,
        y: u16,
        source: LevelFormatError,
    },
}

impl fmt::Display for LevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownLevel { id } => write!(f, "level {id} is not registered"),
            Self::LoadFailed { id, path, source } => {
                write!(f, "failed to load level {id} from {path}: {source}")
            }
            Self::TileReadFailed { id, x, y, source } => {
                write!(f, "failed to read tile ({x}, {y}) in level {id}: {source}")
            }
        }
    }
}

impl Error for LevelError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::UnknownLevel { .. } => None,
            Self::LoadFailed { source, .. } => Some(source),
            Self::TileReadFailed { source, .. } => Some(source),
        }
    }
}

pub fn draw_level(id: u8, assets: &Assets) -> Result<(), LevelError> {
    match id {
        0 => draw_loaded_level(id, &get_level(id)?, assets),
        1 => Ok(()),
        _ => Err(LevelError::UnknownLevel { id }),
    }
}

fn draw_loaded_level(id: u8, level: &Level, assets: &Assets) -> Result<(), LevelError> {
    for y in 0..level.height {
        for x in 0..level.width {
            let tile = level
                .tile(LayerKind::Ground, x, y)
                .map_err(|source| LevelError::TileReadFailed { id, x, y, source })?;

            let x_position = f32::from(x) * f32::from(TILE_SIZE);
            let y_position = f32::from(y) * f32::from(TILE_SIZE);

            if let Some(texture) = level_tile_texture(assets, tile) {
                draw_texture(texture, x_position, y_position, WHITE);
            } else {
                draw_missing_tile(x_position, y_position);
            }
        }
    }

    Ok(())
}

fn level_tile_texture(assets: &Assets, tile: u16) -> Option<&Texture2D> {
    match tile {
        0 => Some(&assets.level.base_tile),
        1 => Some(&assets.level.grass),
        _ => None,
    }
}

fn draw_missing_tile(x_position: f32, y_position: f32) {
    let tile_size = f32::from(TILE_SIZE);
    draw_rectangle(x_position, y_position, tile_size, tile_size, MAGENTA);
    draw_line(
        x_position,
        y_position,
        x_position + tile_size,
        y_position + tile_size,
        2.0,
        BLACK,
    );
    draw_line(
        x_position + tile_size,
        y_position,
        x_position,
        y_position + tile_size,
        2.0,
        BLACK,
    );
}

fn get_level(id: u8) -> Result<Level, LevelError> {
    let path = format!("assets/levels/{id}.tlvl");

    Level::load_from_file(&path).map_err(|source| LevelError::LoadFailed { id, path, source })
}
