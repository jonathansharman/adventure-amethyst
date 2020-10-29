use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

/// Allows an entity to collide with other collider entities.
#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct Collider {
	pub half_width: f32,
	pub half_height: f32,
}

impl Component for Collider {
	type Storage = DenseVecStorage<Self>;
}
