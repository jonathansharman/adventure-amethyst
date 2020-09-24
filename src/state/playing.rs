use crate::{
	component::{
		Direction,
		Hero,
		Terrain,
	},
	resource::Region,
};

use amethyst::{
	assets::{AssetStorage, Loader},
	core::transform::Transform,
	input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
	prelude::*,
	renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
	ui::{Anchor, FontHandle, LineMode, TtfFormat, UiImage, UiText, UiTransform},
};

/// The main gameplay state.
pub struct Playing;

impl SimpleState for Playing {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		// Register required components.
		world.register::<Terrain>();

		// Create player character.
		let mut hero_transform = Transform::default();
		hero_transform.set_translation_xyz(0.0, 0.0, 0.5);
		let hero_sprite = load_hero_sprite(world);
		world
			.create_entity()
			.with(Hero)
			.with(Direction::Down)
			.with(hero_transform)
			.with(hero_sprite)
			.build();

		// Add region.
		let region = Region::new(world, 10, 10);
		world.insert(region);

		create_ui_example(world);
	}

	/// The following events are handled:
	/// - The game state is quit when either the close button is clicked or when the escape key is pressed.
	/// - Any other keypress is simply logged to the console.
	fn handle_event(&mut self, mut _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
		if let StateEvent::Window(event) = &event {
			// Check if the window should be closed
			if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
				return Trans::Quit;
			}

			// Listen to any key events
			if let Some((_key_code, _element_state)) = get_key(&event) {
				//info!("handling key event: {:?}", event);
			}

			// If you're looking for a more sophisticated event handling solution,
			// including key bindings and gamepad support, please have a look at
			// https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
		}

		// Keep going
		Trans::None
	}
}

fn load_hero_sprite(world: &mut World) -> SpriteRender {
	let loader = world.read_resource::<Loader>();
	let texture_handle = loader.load(
		"sprites/hero.png",
		ImageFormat::default(),
		(),
		&world.read_resource::<AssetStorage<Texture>>(),
	);
	let sheet_handle = loader.load(
		"sprites/hero.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&world.read_resource::<AssetStorage<SpriteSheet>>(),
	);
	SpriteRender {
		sprite_sheet: sheet_handle.clone(),
		sprite_number: 0,
	}
}

/// Creates a simple UI background and a UI text label.
pub fn create_ui_example(world: &mut World) {
	// Create background UI element.
	world
		.create_entity()
		.with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
		.with(UiTransform::new(
			"".to_string(),
			Anchor::TopLeft,
			Anchor::TopLeft,
			30.0,
			-30.0,
			0.0,
			250.0,
			50.0,
		))
		.build();

	// This simply loads a font from the asset folder and puts it in the world as a resource,
	// we also get a ref to the font that we then can pass to the text label we crate later.
	let font: FontHandle = world.read_resource::<Loader>().load(
		"fonts/Bangers-Regular.ttf",
		TtfFormat,
		(),
		&world.read_resource(),
	);

	// This creates the actual label and places it on the screen.
	// Take note of the z position given, this ensures the label gets rendered above the background UI element.
	world
		.create_entity()
		.with(UiTransform::new(
			"".to_string(),
			Anchor::TopLeft,
			Anchor::TopLeft,
			40.0,
			-40.0,
			1.0,
			200.0,
			50.0,
		))
		.with(UiText::new(
			font,
			"Hello, Amethyst UI!".to_string(),
			[1.0, 1.0, 1.0, 1.0],
			30.0,
			LineMode::Single,
			Anchor::TopLeft,
		))
		.build();
}
