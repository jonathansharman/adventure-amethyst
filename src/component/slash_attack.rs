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
}
