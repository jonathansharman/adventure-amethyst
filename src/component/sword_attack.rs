use amethyst::ecs::{Component, DenseVecStorage, Entity};

pub struct SwordAttack {
	source_id: Entity,
	is_active: bool,
}

impl Component for SwordAttack {
	type Storage = DenseVecStorage<Self>;
}

impl SwordAttack {
	/// `source_id` The entity ID of the character that spawned this sword attack.
	pub fn new(source_id: Entity) -> Self {
		Self {
			source_id,
			is_active: true,
		}
	}

	/// The entity ID of the character that spawned this sword attack.
	pub fn source_id(&self) -> Entity {
		self.source_id
	}

	/// Whether this sword attack is able to hit a character.
	pub fn is_active(&self) -> bool {
		self.is_active
	}

	/// Makes the sword attack inactive (unable to hit a character).
	pub fn make_inactive(&mut self) {
		self.is_active = false;
	}
}
