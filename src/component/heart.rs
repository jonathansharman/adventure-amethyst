use amethyst::ecs::{Component, DenseVecStorage};

/// A health pickup.
pub struct Heart;

impl Component for Heart {
	type Storage = DenseVecStorage<Self>;
}
