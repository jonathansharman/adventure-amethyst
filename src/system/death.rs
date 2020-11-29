use crate::component::{
	Health,
	removal::TiedToEntity,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
	utils::removal::{Removal, exec_removal},
};

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct Death;

impl<'a> System<'a> for Death {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Removal<TiedToEntity>>,
		ReadStorage<'a, Health>,
	);

	fn run(&mut self, (entities, mut sto_removal_tied_to_entity, sto_health): Self::SystemData) {
		// Kill entities with zero health.
		for (id, health) in (&entities, &sto_health).join() {
			if health.current() == 0 {
				// Delete this entity and any entities that depend on it.
				entities.delete(id).unwrap();
				exec_removal(&entities, &mut sto_removal_tied_to_entity, TiedToEntity(id));
			}
		}
	}
}
