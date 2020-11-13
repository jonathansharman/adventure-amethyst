use crate::{
	component::{
		Animation,
		behavior::Wander,
		collider::RectangleCollider,
		Direction,
		Enemy,
		Frame,
		Health,
		Heart,
		Hero,
		HeroState,
		Position,
		Terrain,
		Velocity,
	},
	constants::*,
	resource::{
		Camera,
		Hud,
		Region,
		SpriteSheets,
		Textures,
	},
};

use amethyst::{
	core::transform::Transform,
	prelude::*,
	renderer::{Camera as AmethystCamera, SpriteRender},
	window::ScreenDimensions,
	winit::{
		dpi::LogicalSize,
		Event,
		WindowEvent,
	},
};

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
		let textures = Textures::new(&world);
		let sprite_sheets = SpriteSheets::new(&world, &textures);

		// Create hero (player character).
		let hero_position = Position { x: TILE_SIZE * 30.0, y: -TILE_SIZE * 30.0 };
		let hero_collider = RectangleCollider {
			half_width: 0.5 * TILE_SIZE,
			half_height: 0.5 * TILE_SIZE,
		};
		let hero = world
			.create_entity()
			.with(Hero { state: HeroState::FreelyMoving })
			.with(Health::new(HERO_BASE_HEALTH))
			.with(hero_position)
			.with(Velocity::default())
			.with(Direction::Down)
			.with(hero_collider)
			.with(Animation::new(vec!(
				Frame {
					up: 0,
					down: 1,
					left: 2,
					right: 3,
					duration: Duration::from_secs(1),
				}
			)))
			.with(Transform::default())
			.with(SpriteRender {
				sprite_sheet: sprite_sheets.hero.clone(),
				sprite_number: 0,
			})
			.build();

		// Create region.
		let mut region = Region::new();

		// Load starting region.
		region.load(
			"test.ron",
			&world.entities(),
			&sprite_sheets,
			&mut world.write_storage::<Terrain>(),
			&mut world.write_storage::<Enemy>(),
			&mut world.write_storage::<Health>(),
			&mut world.write_storage::<Heart>(),
			&mut world.write_storage::<Wander>(),
			&mut world.write_storage::<Position>(),
			&mut world.write_storage::<Velocity>(),
			&mut world.write_storage::<Direction>(),
			&mut world.write_storage::<RectangleCollider>(),
			&mut world.write_storage::<Animation>(),
			&mut world.write_storage::<Transform>(),
			&mut world.write_storage::<SpriteRender>(),
		);

		// Move hero to the region's first entrance.
		region.place_at_entrance(
			hero,
			1,
			&mut world.write_storage::<Position>(),
			&mut world.write_storage::<Direction>(),
		);

		// Insert the region into the world.
		world.insert(region);

		// Insert textures and sprite sheets into the world.
		world.insert(textures);
		world.insert(sprite_sheets);

		// Create and insert HUD.
		world.insert(Hud::new());

		add_camera(world);
	}

	/// The following events are handled:
	/// - The game state is quit when either the close button is clicked or when the escape key is pressed.
	/// - Any other keypress is simply logged to the console.
	fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
		if let StateEvent::Window(Event::WindowEvent { event, .. }) = &event {
			match event {
				WindowEvent::Resized(LogicalSize { width, height }) => {
					let camera = data.world.fetch::<Camera>();
					let mut sto_amethyst_camera = data.world.write_storage::<AmethystCamera>();
					let amethyst_camera = sto_amethyst_camera.get_mut(camera.id).unwrap();
					*amethyst_camera = AmethystCamera::standard_2d(*width as f32, *height as f32);
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
}

/// Adds the camera resource.
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
