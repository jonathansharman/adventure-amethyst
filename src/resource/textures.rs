use amethyst::{
	assets::{AssetStorage, Handle, Loader},
	prelude::*,
	renderer::{
		ImageFormat,
		Texture,
	},
};

/// Container for all the game's textures.
pub struct Textures {
	pub enemy: Handle<Texture>,
	pub hero: Handle<Texture>,
	pub slash_attack: Handle<Texture>,
	pub thrust_attack: Handle<Texture>,
	pub terrain: Handle<Texture>,
	pub hearts: Handle<Texture>,
}

impl Textures {
	pub fn new(world: &World) -> Self {
		let loader = world.read_resource::<Loader>();
		let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
		let load_texture = |name| {
			loader.load(
				format!("sprites/{}.png", name),
				ImageFormat::default(),
				(),
				texture_storage,
			)
		};
		Self {
			enemy: load_texture("arrow"),
			hero: load_texture("arrow"),
			slash_attack: load_texture("slash_attack"),
			thrust_attack: load_texture("thrust_attack"),
			terrain: load_texture("terrain"),
			hearts: load_texture("hearts"),
		}
	}
}