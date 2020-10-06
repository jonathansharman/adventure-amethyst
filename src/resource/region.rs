use crate::{
	component::{
		behavior::Wander,
		Collider,
		Direction,
		Enemy,
		Position,
		Terrain,
		TileCoords,
	},
	constants::*,
};

use amethyst::{
	assets::{AssetStorage, Handle, Loader},
	core::Transform,
	ecs::{Entity, Entities, ReadStorage, WriteStorage},
	prelude::*,
	renderer::{
		ImageFormat,
		SpriteRender,
		SpriteSheet,
		SpriteSheetFormat,
		Texture,
	},
};
use nalgebra::Vector3;
use ron::de::from_reader;
use serde::Deserialize;

use std::fs::File;

pub struct Region {
	terrain_sheet_handle: Handle<SpriteSheet>,
	enemy_sheet_handle: Handle<SpriteSheet>,
	row_count: usize,
	col_count: usize,
	tiles: Vec<Entity>,
	entrances: Vec<Entrance>,
	exits: Vec<Exit>,
	enemies: Vec<Entity>,
}

impl Region {
	/// Creates a new empty region.
	pub fn new(world: &World) -> Region {
		let loader = world.read_resource::<Loader>();
		// Load terrain sprite sheet.
		let terrain_texture_handle;
		let terrain_sheet_handle;
		{
			terrain_texture_handle = loader.load(
				"sprites/terrain.png",
				ImageFormat::default(),
				(),
				&world.read_resource::<AssetStorage<Texture>>(),
			);
			terrain_sheet_handle = loader.load(
				"sprites/terrain.ron",
				SpriteSheetFormat(terrain_texture_handle),
				(),
				&world.read_resource::<AssetStorage<SpriteSheet>>(),
			);
		}
		// Load enemy sprite sheet.
		let enemy_texture_handle;
		let enemy_sheet_handle;
		{
			enemy_texture_handle = loader.load(
				"sprites/arrow.png",
				ImageFormat::default(),
				(),
				&world.read_resource::<AssetStorage<Texture>>(),
			);
			enemy_sheet_handle = loader.load(
				"sprites/arrow.ron",
				SpriteSheetFormat(enemy_texture_handle),
				(),
				&world.read_resource::<AssetStorage<SpriteSheet>>(),
			);
		}
		// Construct empty region.
		Region {
			terrain_sheet_handle,
			enemy_sheet_handle,
			row_count: 0,
			col_count: 0,
			tiles: Vec::new(),
			entrances: Vec::new(),
			exits: Vec::new(),
			enemies: Vec::new(),
		}
	}

	/// Loads region data from `filename` within the `assets/regions` directory.
	pub fn load<'a>(
		&mut self,
		filename: &str,
		entities: &Entities<'a>,
		all_terrain: &mut WriteStorage<'a, Terrain>,
		all_enemies: &mut WriteStorage<'a, Enemy>,
		all_wanders: &mut WriteStorage<'a, Wander>,
		all_positions: &mut WriteStorage<'a, Position>,
		all_directions: &mut WriteStorage<'a, Direction>,
		all_colliders: &mut WriteStorage<'a, Collider>,
		all_transforms: &mut WriteStorage<'a, Transform>,
		all_sprites: &mut WriteStorage<'a, SpriteRender>,
	) {
		// Delete current entities.
		for tile in self.tiles.iter() {
			entities.delete(*tile).unwrap();
		}
		for enemy in self.enemies.iter() {
			entities.delete(*enemy).unwrap();
		}

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
			transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			// Set sprite based on terrain.
			let sprite = SpriteRender {
				sprite_sheet: self.terrain_sheet_handle.clone(),
				sprite_number: terrain as usize,
			};
			// Add the tile to the world and the region's tile list.
			let tile = entities
				.build_entity()
				.with(terrain, all_terrain)
				.with(transform, all_transforms)
				.with(sprite, all_sprites)
				.build();
			tiles.push(tile);
		}
		// Generate enemies.
		let enemies = region_data.enemies.into_iter()
			.map(|enemy_data| {
				let mut transform = Transform::default();
				transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
				let sprite = SpriteRender {
					sprite_sheet: self.enemy_sheet_handle.clone(),
					sprite_number: 0,
				};
				entities
					.build_entity()
					.with(Enemy, all_enemies)
					.with(Wander::default(), all_wanders)
					.with(enemy_data.location.into(), all_positions)
					.with(Direction::Down, all_directions)
					.with(Collider, all_colliders)
					.with(transform, all_transforms)
					.with(sprite, all_sprites)
					.build()
			})
			.collect();
		// Assign to fields.
		self.row_count = row_count;
		self.col_count = col_count;
		self.tiles = tiles;
		self.entrances = region_data.entrances;
		self.exits = region_data.exits;
		self.enemies = enemies;
	}

	/// Places `entity` at the entrance at index `entrance_idx`.
	pub fn place_at_entrance(
		&self,
		entity: Entity,
		entrance_idx: usize,
		all_positions: &mut WriteStorage<Position>,
		all_directions: &mut WriteStorage<Direction>
	) {
		let entrance = &self.entrances[entrance_idx];
		let position = all_positions.get_mut(entity).unwrap();
		position.x = entrance.location.col as f32 * TILE_SIZE;
		position.y = entrance.location.row as f32 * -TILE_SIZE;
		*all_directions.get_mut(entity).unwrap() = entrance.direction;
	}

	/// Causes `entity` to take the exit it is standing on, if any.
	pub fn take_exit<'a>(
		&mut self,
		entity: Entity,
		entities: &Entities<'a>,
		all_positions: &mut WriteStorage<'a, Position>,
		all_directions: &mut WriteStorage<'a, Direction>,
		all_colliders: &mut WriteStorage<'a, Collider>,
		all_terrain: &mut WriteStorage<'a, Terrain>,
		all_enemies: &mut WriteStorage<'a, Enemy>,
		all_wanders: &mut WriteStorage<'a, Wander>,
		all_transforms: &mut WriteStorage<'a, Transform>,
		all_sprites: &mut WriteStorage<'a, SpriteRender>,
	) {
		let position = all_positions.get(entity);
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
					self.load(
						&exit.target_region,
						entities,
						all_terrain,
						all_enemies,
						all_wanders,
						all_positions,
						all_directions,
						all_colliders,
						all_transforms,
						all_sprites,
					);
					self.place_at_entrance(entity, exit.target_entrance_idx, all_positions, all_directions);
					return;
				}
			}
		}
	}

	/// Gets the terrain at the given `row` and `col`, if any.
	pub fn terrain_at_tile_coords(&self, all_terrain: &ReadStorage<Terrain>, tile_coords: TileCoords) -> Option<Terrain> {
		// Ensure coordinates are in bounds.
		if tile_coords.row >= self.row_count || tile_coords.col >= self.col_count {
			return None;
		}
		// Compute index.
		let index = tile_coords.row * self.col_count + tile_coords.col;
		// Get terrain.
		self.tiles.get(index).and_then(|tile| {
			all_terrain.get(*tile).map(|terrain| *terrain)
		})
	}

	/// Gets the terrain at the given (`x`, `y`) coordinates, if any.
	pub fn terrain_at_position(&self, all_terrain: &ReadStorage<Terrain>, position: Position) -> Option<Terrain> {
		let tile_coords: Option<TileCoords> = position.into();
		tile_coords.and_then(|tile_coords| {
			self.terrain_at_tile_coords(all_terrain, tile_coords)
		})
	}
}

/// Used for reading region data from a file.
#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
struct RegionData {
	col_count: usize,
	terrain: Vec<Terrain>,
	entrances: Vec<Entrance>,
	exits: Vec<Exit>,
	enemies: Vec<EnemyData>,
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

/// Used for reading enemy data from a file.
#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
struct EnemyData {
	location: TileCoords,
}
