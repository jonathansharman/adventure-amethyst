use amethyst::{
	ecs::Entity,
};

/// Tracks and displays HUD data, like current health, selected items, etc.
pub struct Hud {
	pub current_hearts: i32,
	pub max_hearts: i32,
	pub heart_images: Vec<Entity>,
}

impl Hud {
	pub fn new() -> Self {
		Self {
			current_hearts: 0,
			max_hearts: 0,
			heart_images: Vec::new(),
		}
	}
}
