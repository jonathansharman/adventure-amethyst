use crate::component::Terrain;

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
				transform.set_translation_xyz(col as f32 * 40.0, row as f32 * 40.0, 0.0);
				let sprite;
				if row == 0 || row == row_count - 1 || col == 0 || col == col_count - 1 {
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

	pub fn terrain_at(&self, terrains: ReadStorage<Terrain>, row: usize, col: usize) -> Option<Terrain> {
		if row * self.col_count + col < self.row_count * self.col_count + self.col_count {
			terrains.get(self.tiles[row * self.col_count + col]).map(|terrain| *terrain)
		} else {
			None
		}
	}
}
