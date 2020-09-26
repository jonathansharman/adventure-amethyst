use amethyst::ecs::{Component, DenseVecStorage};

/// Causes an entity to collide with the environment and other colliders.
pub struct Collider;

impl Component for Collider {
	type Storage = DenseVecStorage<Self>;
}
