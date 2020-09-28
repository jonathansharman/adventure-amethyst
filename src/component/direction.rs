use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize)]
#[repr(usize)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Component for Direction {
	type Storage = DenseVecStorage<Self>;
}
