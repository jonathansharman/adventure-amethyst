use amethyst::ecs::{Component, DenseVecStorage};

/// Tag component for the hero, enemies, etc.
pub struct Character;

impl Component for Character {
	type Storage = DenseVecStorage<Self>;
}
