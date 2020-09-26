use crate::{
	component::Terrain,
	constants::*,
};

use amethyst::{
	assets::{AssetStorage, Loader},
	core::Transform,
	ecs::{Entity, ReadStorage},
	prelude::*,
	renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use ron::de::from_reader;
use serde::Deserialize;

use std::fs::File;

pub struct Region {
	row_count: usize,
	col_count: usize,
	tiles: Vec<Entity>,
}

/// Intermediary type for reading region data from a file.
#[derive(Deserialize)]
struct RegionData {
	col_count: usize,
	terrain: Vec<Terrain>,
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
			let sprite = match terrain {
				Terrain::Floor => SpriteRender {
					sprite_sheet: sheet_handle.clone(),
					sprite_number: 0,
				},
				Terrain::Wall => SpriteRender {
					sprite_sheet: sheet_handle.clone(),
					sprite_number: 1,
				},
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
		}
	}

	/// Gets the terrain at the given `row` and `col`, if any.
	pub fn terrain_at_row_col(&self, terrains: &ReadStorage<Terrain>, row: usize, col: usize) -> Option<Terrain> {
		// Ensure coordinates are in bounds.
		if row >= self.row_count || col >= self.col_count {
			return None;
		}
		// Compute index.
		let index = row * self.col_count + col;
		// Get terrain.
		self.tiles.get(index).and_then(|tile| {
			terrains.get(*tile).map(|terrain| *terrain)
		})
	}

	/// Gets the terrain at the given (`x`, `y`) coordinates, if any.
	pub fn terrain_at_x_y(&self, terrains: &ReadStorage<Terrain>, x: f32, y: f32) -> Option<Terrain> {
		if x < 0.0 || y > 0.0 {
			None
		} else {
			self.terrain_at_row_col(terrains, (y / -TILE_SIZE) as usize, (x / TILE_SIZE) as usize)
		}
	}
}
