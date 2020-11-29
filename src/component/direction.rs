use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;
use rand::{
	distributions::{Distribution, Standard},
	Rng,
};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize)]
#[repr(usize)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn opposite(&self) -> Self {
		match self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
		}
	}
}

impl Component for Direction {
	type Storage = DenseVecStorage<Self>;
}

impl Distribution<Direction> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
		match rng.gen_range(0, 4) {
			0 => Direction::Up,
			1 => Direction::Down,
			2 => Direction::Left,
			_ => Direction::Right,
		}
	}
}
