use crate::component::Direction;

use amethyst::ecs::{Component, DenseVecStorage};

/// Behavior that causes an enemy to wander around randomly.
pub struct Wander {
	pub direction: Option<Direction>,
}

impl Component for Wander {
	type Storage = DenseVecStorage<Self>;
}

impl Default for Wander {
	fn default() -> Self {
		Self { direction: None }
	}
}