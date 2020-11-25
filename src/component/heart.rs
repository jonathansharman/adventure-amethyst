use amethyst::ecs::{Component, DenseVecStorage};

/// A health pickup.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Heart;

impl Component for Heart {
	type Storage = DenseVecStorage<Self>;
}
