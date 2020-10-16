use amethyst::ecs::{Component, DenseVecStorage};

/// Behavior that causes an enemy to wander around randomly.
pub struct Wander;

impl Component for Wander {
	type Storage = DenseVecStorage<Self>;
}
