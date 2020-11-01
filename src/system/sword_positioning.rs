use crate::{
	component::{
		Collider,
		Direction,
		Position,
		SwordAttack,
	},
	constants::*,
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Entities, Join, System, SystemData, WriteStorage},
};

/// Repositions swords next to their bearers.
#[derive(SystemDesc)]
pub struct SwordPositioning;

impl<'a> System<'a> for SwordPositioning {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, SwordAttack>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Collider>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (
		entities,
		sto_sword_attack,
		mut sto_position,
		mut sto_direction,
		mut sto_collider,
		mut sto_transform,
	): Self::SystemData) {
		// Note: we can't borrow all components via join because we also need to borrow source components.
		let components_iter = (&entities, &sto_sword_attack, &mut sto_transform).join();
		for (sword_attack_id, sword_attack, sword_transform) in components_iter {
			// Get source data.
			let source_id = sword_attack.source_id();
			let source_position = sto_position.get(source_id).unwrap().clone();
			let source_direction = sto_direction.get(source_id).unwrap().clone();
			let source_collider = sto_collider.get(source_id).unwrap().clone();
			// Get sword data.
			let mut sword_position = sto_position.get_mut(sword_attack_id).unwrap();
			let sword_direction = sto_direction.get_mut(sword_attack_id).unwrap();
			let mut sword_collider = sto_collider.get_mut(sword_attack_id).unwrap();

			// Update sword direction.
			*sword_direction = source_direction;
			// Update sword position and collider.
			match source_direction {
				Direction::Up => {
					sword_collider.half_width = SWORD_HALF_WIDTH;
					sword_collider.half_height = SWORD_HALF_LENGTH;
					sword_position.x = source_position.x;
					sword_position.y = source_position.y + source_collider.half_height + sword_collider.half_height;
				},
				Direction::Down => {
					sword_collider.half_width = SWORD_HALF_WIDTH;
					sword_collider.half_height = SWORD_HALF_LENGTH;
					sword_position.x = source_position.x;
					sword_position.y = source_position.y - source_collider.half_height - sword_collider.half_height;
				},
				Direction::Left => {
					sword_collider.half_width = SWORD_HALF_LENGTH;
					sword_collider.half_height = SWORD_HALF_WIDTH;
					sword_position.x = source_position.x - source_collider.half_width - sword_collider.half_width;
					sword_position.y = source_position.y;
				},
				Direction::Right => {
					sword_collider.half_width = SWORD_HALF_LENGTH;
					sword_collider.half_height = SWORD_HALF_WIDTH;
					sword_position.x = source_position.x + source_collider.half_width + sword_collider.half_width;
					sword_position.y = source_position.y;
				},
			}
			// Update sword translation.
			sword_transform.set_translation_x(sword_position.x);
			sword_transform.set_translation_y(sword_position.y);
		}
	}
}
