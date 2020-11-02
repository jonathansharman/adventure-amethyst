use crate::component::{
	Health,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entities, Join, System, SystemData, WriteStorage},
};

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct Death;

impl<'a> System<'a> for Death {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Health>,
	);

	fn run(&mut self, (entities, sto_health): Self::SystemData) {
		// Kill entities with zero health.
		for (id, health) in (&entities, &sto_health).join() {
			if health.current() == 0 {
				entities.delete(id).unwrap();
			}
		}
	}
}
