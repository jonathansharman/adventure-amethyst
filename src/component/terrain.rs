use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Terrain {
	Floor,
	Wall,
}

impl Component for Terrain {
	type Storage = DenseVecStorage<Self>;
}
