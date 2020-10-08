use amethyst::ecs::{Component, DenseVecStorage};

/// Causes an entity to collide with the environment and other colliders.
pub struct Collider {
	pub width: f32,
	pub height: f32,
}

impl Component for Collider {
	type Storage = DenseVecStorage<Self>;
}
