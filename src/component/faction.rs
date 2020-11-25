use amethyst::ecs::{Component, DenseVecStorage};

/// A character's relationship to the hero. All characters have a faction, so the
/// presence of this component can be used to check if an entity is a character.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Faction {
	Ally,
	Enemy,
}

impl Component for Faction {
	type Storage = DenseVecStorage<Self>;
}
