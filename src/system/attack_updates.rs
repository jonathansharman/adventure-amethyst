use crate::{
	component::{
		collider::{HalfDiskCollider, RectangleCollider},
		Direction,
		Position,
		SlashAttack,
		ThrustAttack,
	},
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Entities, Join, System, SystemData, WriteStorage},
};

/// Keeps attacks synchronized with their sources.
#[derive(SystemDesc)]
pub struct AttackUpdates;

impl<'a> System<'a> for AttackUpdates {
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
		for (slash_attack_id, slash_attack, slash_attack_transform) in (
			&entities,
			&sto_slash_attack,
			&mut sto_transform,
		).join() {
			let source_id = slash_attack.source_id();
			// Get source data.
			let source_position = *sto_position.get(source_id).unwrap();
			let source_direction = *sto_direction.get(source_id).unwrap();
			let source_collider = sto_rectangle_collider.get(source_id).unwrap();
			// Get slash data.
			let slash_attack_position = sto_position.get_mut(slash_attack_id).unwrap();
			let slash_attack_direction = sto_direction.get_mut(slash_attack_id).unwrap();
			let mut slash_attack_collider = sto_disk_arc_collider.get_mut(slash_attack_id).unwrap();

			// Update slash position and slash/collider direction.
			*slash_attack_direction = source_direction;
			slash_attack_collider.direction = source_direction;
			*slash_attack_position = SlashAttack::compute_position(&source_position, &source_direction, source_collider);
			// Update slash translation.
			slash_attack_transform.set_translation_x(slash_attack_position.x);
			slash_attack_transform.set_translation_y(slash_attack_position.y);
		}
		// Thrust attack updates.
		// Note: we can't borrow all components via join because we also need to borrow source components.
		for (thrust_attack_id, thrust_attack, thrust_attack_transform) in (
			&entities,
			&sto_thrust_attack,
			&mut sto_transform,
		).join() {
			let source_id = thrust_attack.source_id();
			// Get source data.
			let source_position = *sto_position.get(source_id).unwrap();
			let source_direction = *sto_direction.get(source_id).unwrap();
			let source_collider = *sto_rectangle_collider.get(source_id).unwrap();
			// Get thrust data.
			let thrust_attack_position = sto_position.get_mut(thrust_attack_id).unwrap();
			let thrust_attack_direction = sto_direction.get_mut(thrust_attack_id).unwrap();
			let thrust_attack_collider = sto_rectangle_collider.get_mut(thrust_attack_id).unwrap();

			// Update thrust direction.
			*thrust_attack_direction = source_direction;
			// Update thrust position and collider.
			*thrust_attack_position = ThrustAttack::compute_position(&source_position, &source_direction, &source_collider);
			*thrust_attack_collider = ThrustAttack::compute_collider(&source_direction);
			// Update thrust translation.
			thrust_attack_transform.set_translation_x(thrust_attack_position.x);
			thrust_attack_transform.set_translation_y(thrust_attack_position.y);
		}
	}
}
