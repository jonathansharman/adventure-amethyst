use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
}

impl Component for Velocity {
	type Storage = DenseVecStorage<Self>;
}

impl Default for Velocity {
	fn default() -> Velocity {
		Velocity {
			x: 0.0,
			y: 0.0,
		}
	}
}
