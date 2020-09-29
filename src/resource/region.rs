use crate::{
	component::{Direction, Position, Terrain, TileCoords},
	constants::*,
};

use amethyst::{
	assets::Handle,
	core::Transform,
	ecs::{Entity, Entities, ReadStorage, WriteStorage},
	renderer::{SpriteRender, SpriteSheet},
};
use ron::de::from_reader;
use serde::Deserialize;

use std::fs::File;

pub struct Region {
	sheet_handle: Handle<SpriteSheet>,
	row_count: usize,
	col_count: usize,
	tiles: Vec<Entity>,
	entrances: Vec<Entrance>,
	exits: Vec<Exit>,
}

impl Region {
	/// Creates a new empty region.
	pub fn new(sheet_handle: Handle<SpriteSheet>) -> Region {
		Region {
			sheet_handle,
			row_count: 0,
			col_count: 0,
			tiles: Vec::new(),
			entrances: Vec::new(),
			exits: Vec::new(),
		}
	}

	/// Loads region data from `filename` within the `assets/regions` directory.
	pub fn load<'a>(
		&mut self,
		filename: &str,
		entities: &Entities<'a>,
		terrains: &mut WriteStorage<'a, Terrain>,
		transforms: &mut WriteStorage<'a, Transform>,
		sprites: &mut WriteStorage<'a, SpriteRender>,
	) {
		// Load region data from file.
		let path = format!("assets/regions/{}", filename);
		let file = File::open(&path).expect("Could not open region file");
		let region_data: RegionData = from_reader(file).expect("Error in region file");

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
				sprite_sheet: self.sheet_handle.clone(),
				sprite_number: terrain as usize,
			};
			// Add the tile to the world and the region's tile list.
			let tile = entities
				.build_entity()
				.with(terrain, terrains)
				.with(transform, transforms)
				.with(sprite, sprites)
				.build();
			tiles.push(tile);
		}
		// Assign to fields.
		self.row_count = row_count;
		self.col_count = col_count;
		self.tiles = tiles;
		self.entrances = region_data.entrances;
		self.exits = region_data.exits;
	}

	/// Places `entity` at the entrance at index `entrance_idx`.
	pub fn place_at_entrance(
		&self,
		entity: Entity,
		entrance_idx: usize,
		positions: &mut WriteStorage<Position>,
		directions: &mut WriteStorage<Direction>
	) {
		let entrance = &self.entrances[entrance_idx];
		let position = positions.get_mut(entity).unwrap();
		position.x = entrance.location.col as f32 * TILE_SIZE;
		position.y = entrance.location.row as f32 * -TILE_SIZE;
		*directions.get_mut(entity).unwrap() = entrance.direction;
	}

	/// Causes `entity` to take the exit it is standing on, if any.
	pub fn take_exit<'a>(
		&mut self,
		entity: Entity,
		entities: &Entities<'a>,
		positions: &mut WriteStorage<'a, Position>,
		directions: &mut WriteStorage<'a, Direction>,
		terrains: &mut WriteStorage<'a, Terrain>,
		transforms: &mut WriteStorage<'a, Transform>,
		sprites: &mut WriteStorage<'a, SpriteRender>,
	) {
		let position = positions.get(entity);
		let tile_coords: Option<TileCoords> = position.and_then(|position| {
			// Check position of center rather than upper-left corner.
			Position {
				x: position.x + TILE_SIZE / 2.0,
				y: position.y - TILE_SIZE / 2.0
			}.into()
		});
		if let Some(tile_coords) = tile_coords {
			for exit in self.exits.clone() {
				if exit.location == tile_coords {
					self.load(&exit.target_region, entities, terrains, transforms, sprites);
					self.place_at_entrance(entity, exit.target_entrance_idx, positions, directions);
					return;
				}
			}
		}
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

/// Intermediary type for reading region data from a file.
#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
struct RegionData {
	col_count: usize,
	terrain: Vec<Terrain>,
	entrances: Vec<Entrance>,
	exits: Vec<Exit>,
}

/// An entrance to a region.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize)]
struct Entrance {
	location: TileCoords,
	direction: Direction,
}

/// An exit from a region.
#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
struct Exit {
	location: TileCoords,
	target_region: String,
	target_entrance_idx: usize,
}
