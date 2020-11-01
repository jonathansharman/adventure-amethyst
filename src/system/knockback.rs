use crate::component::{
	KnockedBack,
	Velocity,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entity, Entities, Join, System, SystemData, WriteStorage},
};

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct Knockback;

impl<'a> System<'a> for Knockback {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, KnockedBack>,
		WriteStorage<'a, Velocity>,
	);

	fn run(&mut self, (entities, mut sto_knocked_back, mut sto_velocity): Self::SystemData) {
		let mut entities_finished_knockback: Vec<Entity> = Vec::new();
		let components_iter = (&entities, &mut sto_knocked_back, &mut sto_velocity).join();
		for (entity, knocked_back, velocity) in components_iter {
			*velocity = knocked_back.velocity;
			knocked_back.frames_left -= 1;
			if knocked_back.frames_left == 0 {
				entities_finished_knockback.push(entity);
			}
		}
		for entity in entities_finished_knockback {
			sto_knocked_back.remove(entity);
		}
	}
}
