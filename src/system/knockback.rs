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

	fn run(&mut self, (entities, mut all_knocked_backs, mut all_velocities): Self::SystemData) {
		let mut entities_finished_knockback: Vec<Entity> = Vec::new();
		let component_iter = (&entities, &mut all_knocked_backs, &mut all_velocities).join();
		for (entity, knocked_back, velocity) in component_iter {
			*velocity = knocked_back.velocity;
			knocked_back.frames_left -= 1;
			if knocked_back.frames_left == 0 {
				entities_finished_knockback.push(entity);
			}
		}
		for entity in entities_finished_knockback {
			all_knocked_backs.remove(entity);
		}
	}
}
