use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Hero {
	pub state: HeroState,
}

impl Component for Hero {
	type Storage = DenseVecStorage<Self>;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum HeroState {
	FreelyMoving,
	Slashing {
		slash_attack_id: Entity,
		frames_left: u32,
	},
	Thrusting {
		thrust_attack_id: Entity,
		frames_left: u32,
	},
}
