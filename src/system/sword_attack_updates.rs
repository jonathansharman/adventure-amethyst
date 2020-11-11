use crate::{
	component::{
		collider::{HalfDiskCollider, RectangleCollider},
		Direction,
		Position,
		SlashAttack,
		ThrustAttack,
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
pub struct SwordAttackUpdates;

impl<'a> System<'a> for SwordAttackUpdates {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, SlashAttack>,
		WriteStorage<'a, ThrustAttack>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, HalfDiskCollider>,
		WriteStorage<'a, RectangleCollider>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (
		entities,
		sto_slash_attack,
		sto_thrust_attack,
		mut sto_position,
		mut sto_direction,
		mut sto_disk_arc_collider,
		mut sto_rectangle_collider,
		mut sto_transform,
	): Self::SystemData) {
		// Slash attack updates.

		// Note: we can't borrow all components via join because we also need to borrow source components.
		let components_iter = (&entities, &sto_slash_attack, &mut sto_transform).join();
		for (slash_attack_id, slash_attack, sword_transform) in components_iter {
			let source_id = slash_attack.source_id();
			// If source no longer exists, delete this attack.
			if !entities.is_alive(source_id) {
				entities.delete(slash_attack_id).unwrap();
				continue;
			}
			// Get source data.
			let source_position = sto_position.get(source_id).unwrap().clone();
			let source_direction = sto_direction.get(source_id).unwrap().clone();
			let source_collider = sto_rectangle_collider.get(source_id).unwrap().clone();
			// Get slash data.
			let slash_attack_position = sto_position.get_mut(slash_attack_id).unwrap();
			let slash_attack_direction = sto_direction.get_mut(slash_attack_id).unwrap();
			let mut slash_attack_collider = sto_disk_arc_collider.get_mut(slash_attack_id).unwrap();

			// Update slash direction.
			*slash_attack_direction = source_direction;
			slash_attack_collider.direction = source_direction;
			// Update slash position and collider.
			match source_direction {
				Direction::Up => {
					slash_attack_position.x = source_position.x;
					slash_attack_position.y = source_position.y + source_collider.half_height;
				},
				Direction::Down => {
					slash_attack_position.x = source_position.x;
					slash_attack_position.y = source_position.y - source_collider.half_height;
				},
				Direction::Left => {
					slash_attack_position.x = source_position.x - source_collider.half_width;
					slash_attack_position.y = source_position.y;
				},
				Direction::Right => {
					slash_attack_position.x = source_position.x + source_collider.half_width;
					slash_attack_position.y = source_position.y;
				},
			}
			// Update slash translation.
			sword_transform.set_translation_x(slash_attack_position.x);
			sword_transform.set_translation_y(slash_attack_position.y);
		}

		// Thrust attack updates.

		// Note: we can't borrow all components via join because we also need to borrow source components.
		let components_iter = (&entities, &sto_thrust_attack, &mut sto_transform).join();
		for (thrust_attack_id, thrust_attack, sword_transform) in components_iter {
			let source_id = thrust_attack.source_id();
			// If source no longer exists, delete this attack.
			if !entities.is_alive(source_id) {
				entities.delete(thrust_attack_id).unwrap();
				continue;
			}
			// Get source data.
			let source_position = sto_position.get(source_id).unwrap().clone();
			let source_direction = sto_direction.get(source_id).unwrap().clone();
			let source_collider = sto_rectangle_collider.get(source_id).unwrap().clone();
			// Get thrust data.
			let thrust_attack_position = sto_position.get_mut(thrust_attack_id).unwrap();
			let thrust_attack_direction = sto_direction.get_mut(thrust_attack_id).unwrap();
			let thrust_attack_collider = sto_rectangle_collider.get_mut(thrust_attack_id).unwrap();

			// Update thrust direction.
			*thrust_attack_direction = source_direction;
			// Update thrust position and collider.
			match source_direction {
				Direction::Up => {
					*thrust_attack_collider = RectangleCollider { half_width: SWORD_THRUST_HALF_WIDTH, half_height: SWORD_THRUST_HALF_LENGTH };
					thrust_attack_position.x = source_position.x;
					thrust_attack_position.y = source_position.y + source_collider.half_height + thrust_attack_collider.half_height;
				},
				Direction::Down => {
					*thrust_attack_collider = RectangleCollider { half_width: SWORD_THRUST_HALF_WIDTH, half_height: SWORD_THRUST_HALF_LENGTH };
					thrust_attack_position.x = source_position.x;
					thrust_attack_position.y = source_position.y - source_collider.half_height - thrust_attack_collider.half_height;
				},
				Direction::Left => {
					*thrust_attack_collider = RectangleCollider { half_width: SWORD_THRUST_HALF_LENGTH, half_height: SWORD_THRUST_HALF_WIDTH };
					thrust_attack_position.x = source_position.x - source_collider.half_width - thrust_attack_collider.half_width;
					thrust_attack_position.y = source_position.y;
				},
				Direction::Right => {
					*thrust_attack_collider = RectangleCollider { half_width: SWORD_THRUST_HALF_LENGTH, half_height: SWORD_THRUST_HALF_WIDTH };
					thrust_attack_position.x = source_position.x + source_collider.half_width + thrust_attack_collider.half_width;
					thrust_attack_position.y = source_position.y;
				},
			}
			// Update thrust translation.
			sword_transform.set_translation_x(thrust_attack_position.x);
			sword_transform.set_translation_y(thrust_attack_position.y);
		}
	}
}
