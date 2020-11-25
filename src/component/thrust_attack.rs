use crate::{
	component::{Direction, Position, collider::RectangleCollider},
	constants::*,
};

use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct ThrustAttack {
	source_id: Entity,
	is_active: bool,
}

impl Component for ThrustAttack {
	type Storage = DenseVecStorage<Self>;
}

impl ThrustAttack {
	/// `source_id` The entity ID of the character that spawned this thrust attack.
	pub fn new(source_id: Entity) -> Self {
		Self {
			source_id,
			is_active: true,
		}
	}

	/// The entity ID of the character that spawned this thrust attack.
	pub fn source_id(&self) -> Entity {
		self.source_id
	}

	/// Whether this thrust attack is able to hit a character.
	pub fn is_active(&self) -> bool {
		self.is_active
	}

	/// Makes the thrust attack inactive (unable to hit a character).
	pub fn make_inactive(&mut self) {
		self.is_active = false;
	}

	/// Computes the position of the attack based on its source's orientation.
	pub fn compute_position(
		source_position: &Position,
		source_direction: &Direction,
		source_collider: &RectangleCollider,
	) -> Position {
		match source_direction {
			Direction::Up => Position {
				x: source_position.x,
				y: source_position.y + source_collider.half_height + THRUST_ATTACK_HALF_LENGTH,
			},
			Direction::Down => Position {
				x: source_position.x,
				y: source_position.y - source_collider.half_height - THRUST_ATTACK_HALF_LENGTH,
			},
			Direction::Left => Position {
				x: source_position.x - source_collider.half_width - THRUST_ATTACK_HALF_LENGTH,
				y: source_position.y,
			},
			Direction::Right => Position {
				x: source_position.x + source_collider.half_width + THRUST_ATTACK_HALF_LENGTH,
				y: source_position.y,
			},
		}
	}

	/// Computes the collider of the attack based on its source's orientation.
	pub fn compute_collider(source_direction: &Direction) -> RectangleCollider {
		match source_direction {
			Direction::Up | Direction::Down => RectangleCollider {
				half_width: THRUST_ATTACK_HALF_WIDTH,
				half_height: THRUST_ATTACK_HALF_LENGTH,
			},
			Direction::Left | Direction::Right => RectangleCollider {
				half_width: THRUST_ATTACK_HALF_LENGTH,
				half_height: THRUST_ATTACK_HALF_WIDTH,
			},
		}
	}
}
