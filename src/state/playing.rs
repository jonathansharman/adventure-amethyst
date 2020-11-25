use crate::{
	component::{
		Animation,
		behavior::{ShootArrows, Wander},
		collider::RectangleCollider,
		Direction,
		Enemy,
		Faction,
		Frame,
		Health,
		Heart,
		Hero,
		HeroState,
		Position,
		Terrain,
		TileCoords,
		Velocity,
	},
	constants::*,
	resource::{
		Camera,
		CurrentRegion,
		Hud,
		Region,
		RegionData,
		SpriteSheets,
		Textures,
	},
};

use amethyst::{
	core::transform::Transform,
	ecs::{Entity, Join},
	prelude::*,
	renderer::{Camera as AmethystCamera},
	utils::removal::{exec_removal, Removal},
	window::ScreenDimensions,
	winit::{
		dpi::LogicalSize,
		Event,
		WindowEvent,
	},
};
use rand::Rng;
use ron::de::from_reader;

use std::fs::File;
use std::time::Duration;

/// The main gameplay state.
pub struct Playing;

impl SimpleState for Playing {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		
		// Register required components.
		world.register::<Terrain>();
		world.register::<Velocity>();

		// Load textures and sprite sheets.
		world.insert(Textures::new(&world));
		world.insert(SpriteSheets::new(&world));

		// Create a current region manager.
		world.insert(CurrentRegion::new());

		// Create hero (player character).
		let hero_position = Position { x: TILE_SIZE * 30.0, y: -TILE_SIZE * 30.0 };
		let hero_collider = RectangleCollider {
			half_width: 0.5 * TILE_SIZE,
			half_height: 0.5 * TILE_SIZE,
		};
		let hero_sprite_sheet = world.read_resource::<SpriteSheets>().hero.clone();
		let hero = world
			.create_entity()
			.with(Hero { state: HeroState::FreelyMoving })
			.with(Faction::Ally)
			.with(Health::new(HERO_BASE_HEALTH))
			.with(hero_position)
			.with(Velocity::default())
			.with(Direction::Down)
			.with(hero_collider)
			.with(Animation::new(hero_sprite_sheet, vec!(
				Frame {
					up: 0,
					down: 1,
					left: 2,
					right: 3,
					duration: Duration::from_secs(1),
				},
			)))
			.with(Transform::default())
			.build();

		// Load starting region.
		load_region("test.ron", world);

		// Move hero to the region's second entrance.
		place_at_entrance(hero, 1, world);

		// Create and insert HUD.
		world.insert(Hud::new());

		add_camera(world);
	}

	fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
		if let StateEvent::Window(Event::WindowEvent { event, .. }) = &event {
			match event {
				WindowEvent::Resized(LogicalSize { width, height }) => {
					let camera = data.world.fetch::<Camera>();
					let mut sto_amethyst_camera = data.world.write_storage::<AmethystCamera>();
					let amethyst_camera = sto_amethyst_camera.get_mut(camera.id).unwrap();
					// Ensure the width and height are even. This avoids minor texture stretching.
					let width = *width as u32;
					let height = *height as u32;
					let width = width + (width & 1);
					let height = height + (height & 1);
					*amethyst_camera = AmethystCamera::standard_2d(width as f32, height as f32);
				},
				WindowEvent::CloseRequested => {
					return Trans::Quit;
				},
				// Ignore other events.
				_ => {},
			}
		}
		Trans::None
	}

	fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		let world = data.world;
		let exits = world.read_resource::<CurrentRegion>().get().exits().clone();
		// See if a hero is on an exit.
		let mut hero_id_exit = None;
		for (hero_id, _hero, hero_position) in (
			&*world.entities(),
			&world.read_storage::<Hero>(),
			&world.read_storage::<Position>(),
		).join() {
			let tile_coords: Option<TileCoords> = (*hero_position).into();
			if let Some(tile_coords) = tile_coords {
				for exit in &exits {
					if exit.location == tile_coords {
						hero_id_exit = Some((hero_id, exit.clone()));
						break;
					}
				}
			}
			if hero_id_exit.is_some() {
				break;
			}
		}
		// If so, take the exit.
		if let Some((hero_id, exit)) = hero_id_exit {
			// Remove all entities associated with the current region (which uses `Removal<()>`)).
			exec_removal(&*world.entities(), &world.read_storage::<Removal<()>>(), ());
			// Load the target region.
			load_region(&exit.target_region, world);
			// Place the player at the target entrance.
			place_at_entrance(hero_id, exit.target_entrance_idx, world);
			// Reset the hero's state.
			world.write_storage::<Hero>().get_mut(hero_id).unwrap().state = HeroState::FreelyMoving;
		}
		Trans::None
	}
}

/// Adds a camera resource to the world.
fn add_camera(world: &mut World) {
	let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 1.0);
	let camera_id = world
		.create_entity()
		.with(AmethystCamera::standard_2d(dimensions.width(), dimensions.height()))
		.with(transform)
		.build();
	world.insert(Camera { id: camera_id });
}

/// Loads a region from `filename` within the `assets/regions` directory.
fn load_region(filename: &str, world: &mut World) {
	// Load region data from file.
	let path = format!("assets/regions/{}", filename);
	let file = File::open(&path).expect("Could not open region file");
	let region_data: RegionData = from_reader(file).expect("Error in region file");

	// Load the region itself.
	let region = Region::new(
		region_data.col_count,
		region_data.terrain,
		region_data.entrances,
		region_data.exits,
		world,
	);
	world.write_resource::<CurrentRegion>().set(region);

	// Generate enemies.
	for enemy_data in region_data.enemies {
		let enemy_position: Position = enemy_data.location.into();
		let enemy_collider = RectangleCollider {
			half_width: 0.5 * TILE_SIZE,
			half_height: 0.5 * TILE_SIZE,
		};
		let enemy_sprite_sheet = world.read_resource::<SpriteSheets>().enemy.clone();
		world
			.create_entity()
			.with(Removal::new(()))
			.with(Enemy)
			.with(Faction::Enemy)
			.with(Health::new(ENEMY_BASE_HEALTH))
			.with(Wander { direction: rand::thread_rng().gen() })
			.with(ShootArrows::new())
			.with(enemy_position)
			.with(Velocity::default())
			.with(Direction::Down)
			.with(enemy_collider)
			.with(Transform::default())
			.with(Animation::new(enemy_sprite_sheet, vec!(
				Frame {
					up: 0,
					down: 1,
					left: 2,
					right: 3,
					duration: Duration::from_secs(1),
				},
			)))
			.with(Transform::default())
			.build();
	}

	// Generate hearts.
	for heart_location in region_data.heart_locations {
		let heart_position: Position = heart_location.into();
		let heart_collider = RectangleCollider {
			half_width: 0.5 * HEART_WIDTH,
			half_height: 0.5 * HEART_HEIGHT,
		};
		let heart_sprite_sheet = world.read_resource::<SpriteSheets>().hearts.clone();
		world
			.create_entity()
			.with(Removal::new(()))
			.with(Heart)
			.with(heart_position)
			.with(Direction::Down)
			.with(heart_collider)
			.with(Animation::new(heart_sprite_sheet, vec!(
				Frame {
					up: 1,
					down: 1,
					left: 1,
					right: 1,
					duration: Duration::from_secs(1),
				},
			)))
			.with(Transform::default())
			.build();
	}
}

/// Places `entity` at the entrance of the current region at index `entrance_idx`.
fn place_at_entrance(entity: Entity, entrance_idx: usize, world: &mut World) {
	let entrance = world.read_resource::<CurrentRegion>().get().entrances()[entrance_idx];
	let mut sto_position = world.write_storage::<Position>();
	let position = sto_position.get_mut(entity).unwrap();
	position.x = entrance.location.col as f32 * TILE_SIZE;
	position.y = entrance.location.row as f32 * -TILE_SIZE;
	*world.write_storage::<Direction>().get_mut(entity).unwrap() = entrance.direction;
}
