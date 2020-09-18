use crate::component::{
	Direction,
	Hero,
};

use amethyst::{
	assets::{AssetStorage, Loader},
	core::transform::Transform,
	ecs::prelude::Entity,
	input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
	prelude::*,
	renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
	ui::{Anchor, FontHandle, LineMode, TtfFormat, UiImage, UiText, UiTransform},
	window::ScreenDimensions,
};

/// The main gameplay state.
pub struct Playing {
	pub camera: Entity,
}

impl SimpleState for Playing {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		// Load and display test sprites.
		let sprites = load_sprites(world);
		let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
		init_sprites(world, &sprites, &dimensions);

		// Create player character.
		let mut hero_transform = Transform::default();
		hero_transform.set_translation_xyz(dimensions.width() / 2.0, dimensions.height() / 2.0, 0.5);
		let hero_sprite = load_hero_sprite(world);
		world
			.create_entity()
			.with(Hero)
			.with(hero_transform)
			.with(Direction::Down)
			.with(hero_sprite)
			.build();

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

/// Loads and splits the `logo.png` image asset into 3 sprites,
/// which will then be assigned to entities for rendering them.
///
/// The provided `world` is used to retrieve the resource loader.
fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
	let loader = world.read_resource::<Loader>();

	// Load the texture for our sprites. We'll later need to add a handle to this texture
	// to our `SpriteRender`s, so we need to keep a reference to it.
	let texture_handle = loader.load(
		"sprites/logo.png",
		ImageFormat::default(),
		(),
		&world.read_resource::<AssetStorage<Texture>>(),
	);

	// Load the spritesheet definition file, which contains metadata on our spritesheet texture.
	let sheet_handle = loader.load(
		"sprites/logo.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&world.read_resource::<AssetStorage<SpriteSheet>>(),
	);

	// Create our sprite renders. Each will have a handle to the texture
	// that it renders from. The handle is safe to clone, since it just
	// references the asset.
	(0..3)
		.map(|i| SpriteRender {
			sprite_sheet: sheet_handle.clone(),
			sprite_number: i,
		})
		.collect()
}

/// Creates an entity in the `world` for each of the provided `sprites`.
/// They are individually placed around the center of the screen.
fn init_sprites(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
	for (i, sprite) in sprites.iter().enumerate() {
		// Center our sprites around the center of the window
		let x = (i as f32 - 1.) * 100.0 + dimensions.width() * 0.5;
		let y = (i as f32 - 1.) * 100.0 + dimensions.height() * 0.5;
		let mut transform = Transform::default();
		transform.set_translation_xyz(x, y, 0.0);

		// Create an entity for each sprite and attach the `SpriteRender` as
		// well as the transform. If you want to add behaviour to your sprites,
		// you'll want to add a custom `Component` that will identify them, and a
		// `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
		world
			.create_entity()
			.with(sprite.clone())
			.with(transform)
			.build();
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
