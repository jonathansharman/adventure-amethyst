use amethyst::ecs::{Component, DenseVecStorage};

pub struct Hero;

impl Component for Hero {
	type Storage = DenseVecStorage<Self>;
}
