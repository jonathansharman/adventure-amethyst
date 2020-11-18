use crate::{
	component::{
		Position,
		Terrain,
		TileCoords,
	},
	constants::*,
	resource::{
		Entrance,
		Exit,
		SpriteSheets,
	},
};

use amethyst::{
	core::Transform,
	ecs::{Entity, ReadStorage},
	prelude::*,
	renderer::SpriteRender,
	utils::removal::Removal,
};
use nalgebra::Vector3;

/// A set of tiles representing a game region, with links to other regions.
pub struct Region {
	row_count: usize,
	col_count: usize,
	tiles: Vec<Entity>,
	entrances: Vec<Entrance>,
	exits: Vec<Exit>,
}

impl Region {
	pub fn new(
		col_count: usize,
		terrain: Vec<Terrain>,
		entrances: Vec<Entrance>,
		exits: Vec<Exit>,
		world: &mut World,
	) -> Self {
		let mut tiles: Vec<Entity> = Vec::new();
		let row_count = terrain.len() / col_count;
		let col_count = col_count;
		for (i, terrain) in terrain.into_iter().enumerate() {
			// Compute row/column indices.
			let row = i / col_count;
			let col = i % col_count;
			// Set transform.
			let mut tile_transform = Transform::default();
			tile_transform.set_translation_xyz(col as f32 * TILE_SIZE, row as f32 * -TILE_SIZE, 0.0);
			tile_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			// Set sprite based on terrain.
			let sprite = SpriteRender {
				sprite_sheet: world.read_resource::<SpriteSheets>().terrain.clone(),
				sprite_number: terrain as usize,
			};
			// Add the tile to the world and the region's tile list, and track its collisions.
			let tile = world
				.create_entity()
				.with(Removal::new(0))
				.with(terrain)
				.with(tile_transform)
				.with(sprite)
				.build();
			tiles.push(tile);
		}
		Self {
			row_count,
			col_count,
			tiles,
			entrances,
			exits,
		}
	}

	/// The entrances into this region.
	pub fn entrances(&self) -> &Vec<Entrance> {
		&self.entrances
	}

	/// The exits from this region.
	pub fn exits(&self) -> &Vec<Exit> {
		&self.exits
	}

	/// Gets the terrain at the given `row` and `col`, if any.
	pub fn terrain_at_tile_coords(&self, sto_terrain: &ReadStorage<Terrain>, tile_coords: TileCoords) -> Option<Terrain> {
		// Ensure coordinates are in bounds.
		if tile_coords.row >= self.row_count || tile_coords.col >= self.col_count {
			return None;
		}
		// Compute index.
		let index = tile_coords.row * self.col_count + tile_coords.col;
		// Get terrain.
		self.tiles.get(index).and_then(|tile| {
			sto_terrain.get(*tile).map(|terrain| *terrain)
		})
	}

	/// Gets the terrain at the given (`x`, `y`) coordinates, if any.
	pub fn terrain_at_position(&self, sto_terrain: &ReadStorage<Terrain>, position: Position) -> Option<Terrain> {
		let tile_coords: Option<TileCoords> = position.into();
		tile_coords.and_then(|tile_coords| {
			self.terrain_at_tile_coords(sto_terrain, tile_coords)
		})
	}
}

/// Resource for accessing the currently active region.
pub struct CurrentRegion {
	region: Option<Region>,
}

impl CurrentRegion {
	/// Creates an empty current region.
	pub fn new() -> Self {
		Self {
			region: None,
		}
	}

	/// Sets the current region.
	pub fn set(&mut self, region: Region) {
		self.region = Some(region);
	}

	/// Gets a reference to the current region.
	pub fn get(&self) -> &Region {
		self.region.as_ref().unwrap()
	}
}

impl Default for CurrentRegion {
	fn default() -> Self {
		Self::new()
	}
}
