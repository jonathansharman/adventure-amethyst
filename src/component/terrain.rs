use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
#[repr(u32)]
pub enum Terrain {
	Floor,
	Wall,
}

impl Component for Terrain {
	type Storage = DenseVecStorage<Self>;
}
