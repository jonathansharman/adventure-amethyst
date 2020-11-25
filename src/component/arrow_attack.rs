use crate::{
	component::{
		collider::RectangleCollider,
		Direction,
		Faction,
	},
	constants::*,
};

use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct ArrowAttack {
	faction: Faction,
}

impl Component for ArrowAttack {
	type Storage = DenseVecStorage<Self>;
}

impl ArrowAttack {
	/// `faction` The faction of the entity that spawned this arrow attack.
	pub fn new(faction: Faction) -> Self {
		Self { faction }
	}

	/// The faction of the entity that spawned this arrow attack.
	pub fn faction(&self) -> Faction {
		self.faction
	}

	/// Computes the collider of the attack based on its source's orientation.
	pub fn compute_collider(source_direction: &Direction) -> RectangleCollider {
		match source_direction {
			Direction::Up | Direction::Down => RectangleCollider {
				half_width: ARROW_ATTACK_HALF_WIDTH,
				half_height: ARROW_ATTACK_HALF_LENGTH,
			},
			Direction::Left | Direction::Right => RectangleCollider {
				half_width: ARROW_ATTACK_HALF_LENGTH,
				half_height: ARROW_ATTACK_HALF_WIDTH,
			},
		}
	}
}
