use crate::constants::TILE_SIZE;

use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct Position {
	pub x: f32,
	pub y: f32,
}

impl Component for Position {
	type Storage = DenseVecStorage<Self>;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct TileCoords {
	pub row: usize,
	pub col: usize,
}

impl From<TileCoords> for Position {
	fn from(tile_coords: TileCoords) -> Self {
		Self {
			x: tile_coords.col as f32 * TILE_SIZE,
			y: tile_coords.row as f32 * -TILE_SIZE,
		}
	}
}

impl From<Position> for Option<TileCoords> {
	fn from(position: Position) -> Self {
		if position.x < 0.0 || position.y > 0.0 {
			None
		} else {
			Some(TileCoords {
				row: (position.y / -TILE_SIZE) as usize,
				col: (position.x / TILE_SIZE) as usize,
			})
		}
	}
}
