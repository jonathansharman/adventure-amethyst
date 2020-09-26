use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Terrain {
	Floor,
	Wall,
}

impl Component for Terrain {
	type Storage = DenseVecStorage<Self>;
}
