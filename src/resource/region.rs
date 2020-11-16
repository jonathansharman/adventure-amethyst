use crate::{
	component::{
		Animation,
		behavior::Wander,
		Character,
		collider::RectangleCollider,
		Direction,
		Enemy,
		Frame,
		Health,
		Heart,
		Hero,
		HeroState,
		Position,
		SlashAttack,
		Terrain,
		TileCoords,
		ThrustAttack,
		Velocity,
	},
	constants::*,
	resource::SpriteSheets,
};

use amethyst::{
	core::Transform,
	ecs::{Entity, Entities, Join, ReadStorage, WriteStorage},
	renderer::SpriteRender,
};
use nalgebra::Vector3;
use rand::Rng;
use ron::de::from_reader;
use serde::Deserialize;

use std::{
	fs::File,
	time::Duration,
};

pub struct Region {
	row_count: usize,
	col_count: usize,
	tiles: Vec<Entity>,
	entrances: Vec<Entrance>,
	exits: Vec<Exit>,
	enemies: Vec<Entity>,
	hearts: Vec<Entity>,
}

impl Region {
	/// Creates a new empty region.
	pub fn new() -> Self {
		Self {
			row_count: 0,
			col_count: 0,
			tiles: Vec::new(),
			entrances: Vec::new(),
			exits: Vec::new(),
			enemies: Vec::new(),
			hearts: Vec::new(),
		}
	}

	/// Loads region data from `filename` within the `assets/regions` directory.
	pub fn load<'a>(
		&mut self,
		filename: &str,
		entities: &Entities<'a>,
		sprite_sheets: &SpriteSheets,
		sto_animation: &mut WriteStorage<'a, Animation>,
		sto_character: &mut WriteStorage<'a, Character>,
		sto_direction: &mut WriteStorage<'a, Direction>,
		sto_enemy: &mut WriteStorage<'a, Enemy>,
		sto_health: &mut WriteStorage<'a, Health>,
		sto_heart: &mut WriteStorage<'a, Heart>,
		sto_position: &mut WriteStorage<'a, Position>,
		sto_rectangle_collider: &mut WriteStorage<'a, RectangleCollider>,
		sto_sprite_render: &mut WriteStorage<'a, SpriteRender>,
		sto_terrain: &mut WriteStorage<'a, Terrain>,
		sto_transform: &mut WriteStorage<'a, Transform>,
		sto_velocity: &mut WriteStorage<'a, Velocity>,
		sto_wander: &mut WriteStorage<'a, Wander>,
	) {
		// Delete current entities.
		for tile in self.tiles.iter() {
			entities.delete(*tile).unwrap();
		}
		for enemy_id in self.enemies.iter() {
			// This may fail if the enemy is dead; ignore the error.
			entities.delete(*enemy_id).unwrap_or_default();
		}
		for heart_id in self.hearts.iter() {
			// This may fail if the heart has been collected; ignore the error.
			entities.delete(*heart_id).unwrap_or_default();
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
			let mut tile_transform = Transform::default();
			tile_transform.set_translation_xyz(col as f32 * TILE_SIZE, row as f32 * -TILE_SIZE, 0.0);
			tile_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			// Set sprite based on terrain.
			let sprite = SpriteRender {
				sprite_sheet: sprite_sheets.terrain.clone(),
				sprite_number: terrain as usize,
			};
			// Add the tile to the world and the region's tile list, and track its collisions.
			let tile = entities
				.build_entity()
				.with(terrain, sto_terrain)
				.with(tile_transform, sto_transform)
				.with(sprite, sto_sprite_render)
				.build();
			tiles.push(tile);
		}
		// Generate enemies.
		let enemies = region_data.enemies.into_iter()
			.map(|enemy_data| {
				let sprite = SpriteRender {
					sprite_sheet: sprite_sheets.enemy.clone(),
					sprite_number: 0,
				};
				let enemy_position: Position = enemy_data.location.into();
				let enemy_collider = RectangleCollider {
					half_width: 0.5 * TILE_SIZE,
					half_height: 0.5 * TILE_SIZE,
				};
				let enemy = entities
					.build_entity()
					.with(Enemy, sto_enemy)
					.with(Character, sto_character)
					.with(Health::new(ENEMY_BASE_HEALTH), sto_health)
					.with(Wander { direction: rand::thread_rng().gen() }, sto_wander)
					.with(enemy_position, sto_position)
					.with(Velocity::default(), sto_velocity)
					.with(Direction::Down, sto_direction)
					.with(enemy_collider, sto_rectangle_collider)
					.with(Animation::new(vec!(
						Frame {
							up: 0,
							down: 1,
							left: 2,
							right: 3,
							duration: Duration::from_secs(1),
						}
					)), sto_animation)
					.with(Transform::default(), sto_transform)
					.with(sprite, sto_sprite_render)
					.build();
				enemy
			})
			.collect();
		// Generate hearts.
		let hearts = region_data.heart_locations.into_iter()
			.map(|heart_location| {
				let sprite = SpriteRender {
					sprite_sheet: sprite_sheets.hearts.clone(),
					sprite_number: 1,
				};
				let heart_position: Position = heart_location.into();
				let heart_collider = RectangleCollider {
					half_width: 0.5 * HEART_WIDTH,
					half_height: 0.5 * HEART_HEIGHT,
				};
				let heart = entities
					.build_entity()
					.with(Heart, sto_heart)
					.with(heart_position, sto_position)
					.with(Direction::Down, sto_direction)
					.with(heart_collider, sto_rectangle_collider)
					.with(Animation::new(vec!(
						Frame {
							up: 1,
							down: 1,
							left: 1,
							right: 1,
							duration: Duration::from_secs(1),
						}
					)), sto_animation)
					.with(Transform::default(), sto_transform)
					.with(sprite, sto_sprite_render)
					.build();
				heart
			})
			.collect();
		// Assign to fields.
		self.row_count = row_count;
		self.col_count = col_count;
		self.tiles = tiles;
		self.entrances = region_data.entrances;
		self.exits = region_data.exits;
		self.enemies = enemies;
		self.hearts = hearts;
	}

	/// Places `entity` at the entrance at index `entrance_idx`.
	pub fn place_at_entrance(
		&self,
		entity: Entity,
		entrance_idx: usize,
		sto_position: &mut WriteStorage<Position>,
		sto_direction: &mut WriteStorage<Direction>
	) {
		let entrance = &self.entrances[entrance_idx];
		let position = sto_position.get_mut(entity).unwrap();
		position.x = entrance.location.col as f32 * TILE_SIZE;
		position.y = entrance.location.row as f32 * -TILE_SIZE;
		*sto_direction.get_mut(entity).unwrap() = entrance.direction;
	}

	/// Causes a hero to take the exit it is standing on, if any.
	pub fn take_exit<'a>(
		&mut self,
		hero_id: Entity,
		hero: &mut Hero,
		entities: &Entities<'a>,
		sprite_sheets: &SpriteSheets,
		sto_animation: &mut WriteStorage<'a, Animation>,
		sto_character: &mut WriteStorage<'a, Character>,
		sto_direction: &mut WriteStorage<'a, Direction>,
		sto_enemy: &mut WriteStorage<'a, Enemy>,
		sto_health: &mut WriteStorage<'a, Health>,
		sto_heart: &mut WriteStorage<'a, Heart>,
		sto_position: &mut WriteStorage<'a, Position>,
		sto_rectangle_collider: &mut WriteStorage<'a, RectangleCollider>,
		sto_slash_attack: &ReadStorage<'a, SlashAttack>,
		sto_sprite_render: &mut WriteStorage<'a, SpriteRender>,
		sto_terrain: &mut WriteStorage<'a, Terrain>,
		sto_thrust_attack: &ReadStorage<'a, ThrustAttack>,
		sto_transform: &mut WriteStorage<'a, Transform>,
		sto_velocity: &mut WriteStorage<'a, Velocity>,
		sto_wander: &mut WriteStorage<'a, Wander>,
	) {
		let position = sto_position.get(hero_id);
		let tile_coords: Option<TileCoords> = position.and_then(|position| (*position).into());
		if let Some(tile_coords) = tile_coords {
			for exit in self.exits.clone() {
				if exit.location == tile_coords {
					self.load(
						&exit.target_region,
						entities,
						sprite_sheets,
						sto_animation,
						sto_character,
						sto_direction,
						sto_enemy,
						sto_health,
						sto_heart,
						sto_position,
						sto_rectangle_collider,
						sto_sprite_render,
						sto_terrain,
						sto_transform,
						sto_velocity,
						sto_wander,
					);
					self.place_at_entrance(hero_id, exit.target_entrance_idx, sto_position, sto_direction);
					// Clear any attacks and reset the hero's state.
					for (slash_attack_id, _slash_attack) in (entities, sto_slash_attack).join() {
						entities.delete(slash_attack_id).unwrap();
					}
					for (thrust_attack_id, _thrust_attack) in (entities, sto_thrust_attack).join() {
						entities.delete(thrust_attack_id).unwrap();
					}
					hero.state = HeroState::FreelyMoving;
					return;
				}
			}
		}
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

impl Default for Region {
	fn default() -> Self {
		Self::new()
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
	heart_locations: Vec<TileCoords>,
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
