use amethyst::ecs::{Component, DenseVecStorage, Entity};

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
}
