use crate::{
	component::{
		collider::{HalfDiskCollider, RectangleCollider},
		Direction,
		Position,
		Shield,
		SlashAttack,
		ThrustAttack,
	},
};

use amethyst::{
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
		WriteStorage<'a, Shield>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, HalfDiskCollider>,
		WriteStorage<'a, RectangleCollider>,
	);

	fn run(&mut self, (
		entities,
		sto_slash_attack,
		sto_thrust_attack,
		sto_shield,
		mut sto_position,
		mut sto_direction,
		mut sto_disk_arc_collider,
		mut sto_rectangle_collider,
	): Self::SystemData) {
		// Slash attack updates.
		for (slash_attack_id, slash_attack) in (
			&entities,
			&sto_slash_attack,
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

			// Update slash direction, position, and collider.
			*slash_attack_direction = source_direction;
			*slash_attack_position = SlashAttack::compute_position(&source_position, &source_direction, source_collider);
			slash_attack_collider.direction = source_direction;
		}
		// Thrust attack updates.
		for (thrust_attack_id, thrust_attack) in (
			&entities,
			&sto_thrust_attack,
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

			// Update thrust direction, position, and collider.
			*thrust_attack_direction = source_direction;
			*thrust_attack_position = ThrustAttack::compute_position(&source_position, &source_direction, &source_collider);
			*thrust_attack_collider = ThrustAttack::compute_collider(&source_direction);
		}
		// Shield updates.
		for (shield_id, shield) in (
			&entities,
			&sto_shield,
		).join() {
			let bearer_id = shield.bearer_id();
			// Get bearer data.
			let bearer_position = *sto_position.get(bearer_id).unwrap();
			let bearer_direction = *sto_direction.get(bearer_id).unwrap();
			let bearer_collider = *sto_rectangle_collider.get(bearer_id).unwrap();
			// Get shield data.
			let shield_position = sto_position.get_mut(shield_id).unwrap();
			let shield_direction = sto_direction.get_mut(shield_id).unwrap();
			let shield_collider = sto_rectangle_collider.get_mut(shield_id).unwrap();

			// Update shield direction, position, and collider.
			*shield_direction = bearer_direction;
			*shield_position = Shield::compute_position(&bearer_position, &bearer_direction, &bearer_collider);
			*shield_collider = Shield::compute_collider(&bearer_direction);
		}
	}
}
