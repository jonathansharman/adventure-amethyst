use crate::component::{
	KnockedBack,
	KnockedBackFinished,
	Velocity,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entity, Entities, Join, System, SystemData, WriteStorage},
};

/// Moves characters that have been knocked back, stops them when the effect expires.
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
		for (id, knocked_back, velocity) in (
			&entities,
			&mut sto_knocked_back,
			&mut sto_velocity,
		).join() {
			if let KnockedBackFinished::Yes = knocked_back.update(velocity) {
				entities_finished_knockback.push(id);
			}
		}
		for id in entities_finished_knockback {
			sto_knocked_back.remove(id);
		}
	}
}
