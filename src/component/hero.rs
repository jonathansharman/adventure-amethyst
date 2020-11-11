use amethyst::ecs::{Component, DenseVecStorage, Entity};

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

pub struct Hero {
	pub state: HeroState,
}

impl Component for Hero {
	type Storage = DenseVecStorage<Self>;
}
