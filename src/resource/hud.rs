use amethyst::{
	ecs::Entity,
};

/// Tracks and displays HUD data, like current health, selected items, etc.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Hud {
	pub max_hearts: i32,
	pub heart_images: Vec<Entity>,
}

impl Hud {
	pub fn new() -> Self {
		Self {
			max_hearts: 0,
			heart_images: Vec::new(),
		}
	}
}
