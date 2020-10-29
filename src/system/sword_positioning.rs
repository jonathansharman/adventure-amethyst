use crate::{
	component::{
		Collider,
		Direction,
		Position,
		Sword,
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
		WriteStorage<'a, Sword>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Collider>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (entities, all_swords, mut all_positions, mut all_directions, mut all_colliders, mut all_transforms): Self::SystemData) {
		// Note: we can't borrow all components via join because we also need to borrow source components.
		let components_iter = (&entities, &all_swords, &mut all_transforms).join();
		for (sword_id, sword, sword_transform) in components_iter {
			// Get source data.
			let source_id = sword.source_id;
			let source_position = all_positions.get(source_id).unwrap().clone();
			let source_direction = all_directions.get(source_id).unwrap().clone();
			let source_collider = all_colliders.get(source_id).unwrap().clone();
			// Get sword data.
			let mut sword_position = all_positions.get_mut(sword_id).unwrap();
			let sword_direction = all_directions.get_mut(sword_id).unwrap();
			let mut sword_collider = all_colliders.get_mut(sword_id).unwrap();

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
