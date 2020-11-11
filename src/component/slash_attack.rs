use amethyst::ecs::{Component, DenseVecStorage, Entity};

pub struct SlashAttack {
	source_id: Entity,
}

impl Component for SlashAttack {
	type Storage = DenseVecStorage<Self>;
}

impl SlashAttack {
	/// `source_id` The entity ID of the character that spawned this slash attack.
	pub fn new(source_id: Entity) -> Self {
		Self { source_id }
	}

	/// The entity ID of the character that spawned this slash attack.
	pub fn source_id(&self) -> Entity {
		self.source_id
	}
}
