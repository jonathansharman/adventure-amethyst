use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Enemy;

impl Component for Enemy {
	type Storage = DenseVecStorage<Self>;
}
