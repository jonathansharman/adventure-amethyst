use amethyst::{
	assets::{AssetStorage, Handle, Loader},
	prelude::*,
	renderer::{
		ImageFormat,
		SpriteSheet,
		SpriteSheetFormat,
		Texture,
	},
};

/// Container for all the game's sprite sheets and UI textures.
pub struct SpriteSheets {
	pub enemy: Handle<SpriteSheet>,
	pub hero: Handle<SpriteSheet>,
	pub sword: Handle<SpriteSheet>,
	pub terrain: Handle<SpriteSheet>,
	pub icons: Handle<Texture>,
}

impl SpriteSheets {
	/// Loads all sprite sheets into a new sprite sheet container.
	pub fn new(world: &World) -> Self {
		let loader = world.read_resource::<Loader>();
		let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
		let sprite_sheet_storage = &world.read_resource::<AssetStorage<SpriteSheet>>();
		let load_texture = |name| {
			loader.load(
				format!("sprites/{}.png", name),
				ImageFormat::default(),
				(),
				texture_storage,
			)
		};
		let load_sprite_sheet = |name| {
			let texture_handle = load_texture(name);
			loader.load(
				format!("sprites/{}.ron", name),
				SpriteSheetFormat(texture_handle),
				(),
				sprite_sheet_storage,
			)
		};
		SpriteSheets {
			enemy: load_sprite_sheet("arrow"),
			hero: load_sprite_sheet("arrow"),
			sword: load_sprite_sheet("sword"),
			terrain: load_sprite_sheet("terrain"),
			icons: load_texture("icons"),
		}
	}
}
