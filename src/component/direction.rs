use amethyst::ecs::{Component, DenseVecStorage};

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Component for Direction {
	type Storage = DenseVecStorage<Self>;
}
