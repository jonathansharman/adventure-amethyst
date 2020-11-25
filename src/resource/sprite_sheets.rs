use crate::resource::Textures;

use amethyst::{
	assets::{AssetStorage, Handle, Loader},
	prelude::*,
	renderer::{
		SpriteSheet,
		SpriteSheetFormat,
	},
};

/// Container for all the game's sprite sheets.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpriteSheets {
	pub enemy: Handle<SpriteSheet>,
	pub hero: Handle<SpriteSheet>,
	pub slash_attack: Handle<SpriteSheet>,
	pub thrust_attack: Handle<SpriteSheet>,
	pub terrain: Handle<SpriteSheet>,
	pub hearts: Handle<SpriteSheet>,
	pub arrow_attack: Handle<SpriteSheet>,
}

impl SpriteSheets {
	pub fn new(world: &World) -> Self {
		let textures = world.read_resource::<Textures>();
		let loader = world.read_resource::<Loader>();
		let sprite_sheet_storage = &world.read_resource::<AssetStorage<SpriteSheet>>();
		let load_sprite_sheet = |name, texture_handle| {
			loader.load(
				format!("sprites/{}.ron", name),
				SpriteSheetFormat(texture_handle),
				(),
				sprite_sheet_storage,
			)
		};
		Self {
			enemy: load_sprite_sheet("character", textures.enemy.clone()),
			hero: load_sprite_sheet("character", textures.hero.clone()),
			slash_attack: load_sprite_sheet("slash_attack", textures.slash_attack.clone()),
			thrust_attack: load_sprite_sheet("thrust_attack", textures.thrust_attack.clone()),
			terrain: load_sprite_sheet("terrain", textures.terrain.clone()),
			hearts: load_sprite_sheet("hearts", textures.hearts.clone()),
			arrow_attack: load_sprite_sheet("arrow_attack", textures.arrow_attack.clone()),
		}
	}
}
