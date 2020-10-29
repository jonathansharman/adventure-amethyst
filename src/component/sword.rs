use amethyst::ecs::{Component, DenseVecStorage, Entity};

pub struct Sword {
	pub source_id: Entity,
}

impl Component for Sword {
	type Storage = DenseVecStorage<Self>;
}
