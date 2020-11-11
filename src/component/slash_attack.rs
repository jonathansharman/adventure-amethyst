use crate::component::{Direction, Position, collider::RectangleCollider};

use amethyst::ecs::{Component, DenseVecStorage, Entity};

use std::collections::HashSet;

pub struct SlashAttack {
	source_id: Entity,
	/// IDs of characters this attack has already damaged.
	already_hit_ids: HashSet<Entity>,
}

impl Component for SlashAttack {
	type Storage = DenseVecStorage<Self>;
}

impl SlashAttack {
	/// `source_id` The entity ID of the character that spawned this slash attack.
	pub fn new(source_id: Entity) -> Self {
		Self {
			source_id,
			already_hit_ids: HashSet::new(),
		}
	}

	/// The entity ID of the character that spawned this slash attack.
	pub fn source_id(&self) -> Entity {
		self.source_id
	}

	/// Checks whether the entity with the given ID has already been hit by this slash attack.
	pub fn has_been_hit(&self, id: Entity) -> bool {
		self.already_hit_ids.contains(&id)
	}

	/// Marks the entity with the given ID as having been hit by this slash attack.
	pub fn mark_as_hit(&mut self, id: Entity) {
		self.already_hit_ids.insert(id);
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
				y: source_position.y + source_collider.half_height,
			},
			Direction::Down => Position {
				x: source_position.x,
				y: source_position.y - source_collider.half_height,
			},
			Direction::Left => Position {
				x: source_position.x - source_collider.half_width,
				y: source_position.y,
			},
			Direction::Right => Position {
				x: source_position.x + source_collider.half_width,
				y: source_position.y,
			},
		}
	}
}
