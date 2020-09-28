use crate::{
	component::{Direction, Position, Terrain, TileCoords},
	constants::*,
};

use amethyst::{
	assets::{AssetStorage, Loader},
	core::Transform,
	ecs::{Entity, ReadStorage, WriteStorage},
	prelude::*,
	renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use ron::de::from_reader;
use serde::Deserialize;

use std::fs::File;

#[derive(Deserialize)]
struct Entrance {
	row: usize,
	col: usize,
	direction: Direction,
}

pub struct Region {
	row_count: usize,
	col_count: usize,
	tiles: Vec<Entity>,
	entrances: Vec<Entrance>,
}

/// Intermediary type for reading region data from a file.
#[derive(Deserialize)]
struct RegionData {
	col_count: usize,
	terrain: Vec<Terrain>,
	entrances: Vec<Entrance>,
}

impl Region {
	/// Loads a region from `filename` within the `assets/regions` directory.
	pub fn load(world: &mut World, filename: &str) -> Region {
		// Load region data from file.
		let path = format!("assets/regions/{}", filename);
		let file = File::open(&path).expect("Could not open region file.");
		let region_data: RegionData = from_reader(file).unwrap();
		// Set up terrain texture and sprite sheet.
		let texture_handle;
		let sheet_handle;
		{
			let loader = world.read_resource::<Loader>();
			texture_handle = loader.load(
				"sprites/terrain.png",
				ImageFormat::default(),
				(),
				&world.read_resource::<AssetStorage<Texture>>(),
			);
			sheet_handle = loader.load(
				"sprites/terrain.ron",
				SpriteSheetFormat(texture_handle),
				(),
				&world.read_resource::<AssetStorage<SpriteSheet>>(),
			);
		}
		// Generate tiles from terrain data.
		let mut tiles: Vec<Entity> = Vec::new();
		let row_count = region_data.terrain.len() / region_data.col_count;
		let col_count = region_data.col_count;
		for (i, terrain) in region_data.terrain.into_iter().enumerate() {
			// Compute row/column indices.
			let row = i / col_count;
			let col = i % col_count;
			// Set transform.
			let mut transform = Transform::default();
			transform.set_translation_xyz(col as f32 * TILE_SIZE, row as f32 * -TILE_SIZE, 0.0);
			// Set sprite based on terrain.
			let sprite = SpriteRender {
				sprite_sheet: sheet_handle.clone(),
				sprite_number: terrain as usize,
			};
			// Add the tile to the world and the region's tile list.
			let tile = world
				.create_entity()
				.with(terrain)
				.with(transform)
				.with(sprite)
				.build();
			tiles.push(tile);
		}
		Region {
			row_count,
			col_count,
			tiles,
			entrances: region_data.entrances,
		}
	}

	pub fn place_at_entrance(
		&self,
		entity: Entity,
		entrance_idx: usize,
		positions: &mut WriteStorage<Position>,
		directions: &mut WriteStorage<Direction>
	) {
		let entrance = &self.entrances[entrance_idx];
		let position = positions.get_mut(entity).unwrap();
		position.x = entrance.col as f32 * TILE_SIZE;
		position.y = entrance.row as f32 * -TILE_SIZE;
		*directions.get_mut(entity).unwrap() = entrance.direction;
	}

	/// Gets the terrain at the given `row` and `col`, if any.
	pub fn terrain_at_tile_coords(&self, terrains: &ReadStorage<Terrain>, tile_coords: TileCoords) -> Option<Terrain> {
		// Ensure coordinates are in bounds.
		if tile_coords.row >= self.row_count || tile_coords.col >= self.col_count {
			return None;
		}
		// Compute index.
		let index = tile_coords.row * self.col_count + tile_coords.col;
		// Get terrain.
		self.tiles.get(index).and_then(|tile| {
			terrains.get(*tile).map(|terrain| *terrain)
		})
	}

	/// Gets the terrain at the given (`x`, `y`) coordinates, if any.
	pub fn terrain_at_position(&self, terrains: &ReadStorage<Terrain>, position: Position) -> Option<Terrain> {
		let tile_coords: Option<TileCoords> = position.into();
		tile_coords.and_then(|tile_coords| {
			self.terrain_at_tile_coords(terrains, tile_coords)
		})
	}
}
