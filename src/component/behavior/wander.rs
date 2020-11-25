use crate::component::Direction;

use amethyst::ecs::{Component, DenseVecStorage};

/// Behavior that causes an enemy to wander around randomly.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Wander {
	pub direction: Option<Direction>,
}

impl Component for Wander {
	type Storage = DenseVecStorage<Self>;
}
