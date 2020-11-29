use crate::{
	component::{Direction, Position, collider::RectangleCollider},
	constants::*,
};

use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Shield {
	bearer_id: Entity,
}

impl Component for Shield {
	type Storage = DenseVecStorage<Self>;
}

impl Shield {
	/// `bearer_id` The entity ID of the character that bears this shield.
	pub fn new(bearer_id: Entity) -> Self {
		Self { bearer_id }
	}

	/// The entity ID of the character that bears this shield.
	pub fn bearer_id(&self) -> Entity {
		self.bearer_id
	}

	/// Computes the position of the attack based on its bearer's orientation.
	pub fn compute_position(
		bearer_position: &Position,
		bearer_direction: &Direction,
		bearer_collider: &RectangleCollider,
	) -> Position {
		match bearer_direction {
			Direction::Up => Position {
				x: bearer_position.x,
				y: bearer_position.y + bearer_collider.half_height + SHIELD_HALF_WIDTH,
			},
			Direction::Down => Position {
				x: bearer_position.x,
				y: bearer_position.y - bearer_collider.half_height - SHIELD_HALF_WIDTH,
			},
			Direction::Left => Position {
				x: bearer_position.x - bearer_collider.half_width - SHIELD_HALF_WIDTH,
				y: bearer_position.y,
			},
			Direction::Right => Position {
				x: bearer_position.x + bearer_collider.half_width + SHIELD_HALF_WIDTH,
				y: bearer_position.y,
			},
		}
	}

	/// Computes the collider of the attack based on its bearer's orientation.
	pub fn compute_collider(bearer_direction: &Direction) -> RectangleCollider {
		match bearer_direction {
			Direction::Up | Direction::Down => RectangleCollider {
				half_width: SHIELD_HALF_LENGTH,
				half_height: SHIELD_HALF_WIDTH,
			},
			Direction::Left | Direction::Right => RectangleCollider {
				half_width: SHIELD_HALF_WIDTH,
				half_height: SHIELD_HALF_LENGTH,
			},
		}
	}
}
