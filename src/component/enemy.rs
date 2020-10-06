use amethyst::ecs::{Component, DenseVecStorage};

pub struct Enemy;

impl Component for Enemy {
	type Storage = DenseVecStorage<Self>;
}
