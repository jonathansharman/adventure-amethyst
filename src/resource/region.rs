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

pub struct Region {
	row_count: usize,
	col_count: usize,
	tiles: Vec<Entity>,
}

impl Region {
	pub fn new(world: &mut World, row_count: usize, col_count: usize) -> Region {
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

		let mut tiles: Vec<Entity> = Vec::new();
		for row in 0..row_count {
			for col in 0..col_count {
				let terrain;
				let mut transform = Transform::default();
				transform.set_translation_xyz(col as f32 * TILE_SIZE, row as f32 * TILE_SIZE, 0.0);
				let sprite;
				let is_edge = row == 0 || row == row_count - 1 || col == 0 || col == col_count - 1;
				let is_center = row == row_count / 2 && col == col_count / 2;
				if is_edge || is_center {
					terrain = Terrain::Wall;
					sprite = SpriteRender {
						sprite_sheet: sheet_handle.clone(),
						sprite_number: 1,
					};
				} else {
					terrain = Terrain::Floor;
					sprite = SpriteRender {
						sprite_sheet: sheet_handle.clone(),
						sprite_number: 0,
					};
				}
				let tile = world
					.create_entity()
					.with(terrain)
					.with(transform)
					.with(sprite)
					.build();
				tiles.push(tile);
			}
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
		if x < 0.0 || y < 0.0 {
			None
		} else {
			self.terrain_at_row_col(terrains, (y / TILE_SIZE) as usize, (x / TILE_SIZE) as usize)
		}
	}
}
