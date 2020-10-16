use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

/// Allows an entity to collide with other collider entities.
#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct Collider {
	pub width: f32,
	pub height: f32,
	pub mobility: Mobility,
}

impl Component for Collider {
	type Storage = DenseVecStorage<Self>;
}

/// Indicates whether a collider might move. Static colliders cannot collide with each other.
#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub enum Mobility {
	Static,
	Dynamic,
}
