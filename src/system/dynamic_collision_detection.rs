use crate::{
	component::{
		ArrowAttack,
		collider::{
			HalfDiskCollider,
			RectangleCollider,
			rect_intersects_half_disk,
			rect_rect_intersection_area,
		},
		Enemy,
		Faction,
		Health,
		Heart,
		Hero,
		Invulnerable,
		KnockedBack,
		Position,
		SlashAttack,
		ThrustAttack,
	},
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

/// Does collision detection and response between moving bodies.
#[derive(SystemDesc)]
pub struct DynamicCollisionDetection;

impl<'a> System<'a> for DynamicCollisionDetection {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, RectangleCollider>,
		ReadStorage<'a, HalfDiskCollider>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, Hero>,
		ReadStorage<'a, Enemy>,
		ReadStorage<'a, Faction>,
		WriteStorage<'a, SlashAttack>,
		WriteStorage<'a, ThrustAttack>,
		WriteStorage<'a, Health>,
		WriteStorage<'a, Heart>,
		WriteStorage<'a, KnockedBack>,
		WriteStorage<'a, Invulnerable>,
		ReadStorage<'a, ArrowAttack>,
	);

	fn run(&mut self, (
		entities,
		sto_rectangle_collider,
		sto_half_disk_collider,
		sto_position,
		sto_hero,
		sto_enemy,
		sto_faction,
		mut sto_slash_attack,
		mut sto_thrust_attack,
		mut sto_health,
		sto_heart,
		mut sto_knocked_back,
		mut sto_invulnerable,
		sto_arrow_attack,
	): Self::SystemData) {
		// Handle thrust attacks against enemies.
		let mut max_overlap_area = 0.0;
		let mut closest_colliding_enemy_id_and_position = None;
		for (
			thrust_attack,
			thrust_attack_collider,
			thrust_attack_position,
		) in (
			&mut sto_thrust_attack,
			&sto_rectangle_collider,
			&sto_position,
		).join() {
			// Ignore thrust attack collisions if it's inactive.
			if !thrust_attack.is_active() {
				continue;
			}
			// Find the closest enemy in collision with the thrust attack.
			for (enemy_id, _enemy, enemy_collider, enemy_position) in (
				&entities,
				&sto_enemy,
				&sto_rectangle_collider,
				&sto_position,
			).join() {
				let overlap_area = rect_rect_intersection_area(
					(&thrust_attack_collider, &thrust_attack_position),
					(&enemy_collider, &enemy_position),
				);
				if overlap_area > max_overlap_area {
					max_overlap_area = overlap_area;
					closest_colliding_enemy_id_and_position = Some((enemy_id, enemy_position));
				}
			}
			// If any enemies were close enough for a collision, damage and knock back the closest one.
			if let Some((enemy_id, enemy_position)) = closest_colliding_enemy_id_and_position {
				// Damage and knock back enemy.
				sto_knocked_back.insert(enemy_id, KnockedBack::from_positions(thrust_attack_position, enemy_position)).unwrap();
				sto_health.get_mut(enemy_id).unwrap().damage(1);
				// Make the thrust attack inactive now that it has hit an enemy.
				thrust_attack.make_inactive();
			}
		}
		// Handle slash attacks against enemies.
		for (slash_attack, slash_attack_collider, slash_attack_position) in (
			&mut sto_slash_attack,
			&sto_half_disk_collider,
			&sto_position,
		).join() {
			// Damage and knock back any enemies in collision with the slash attack.
			for (enemy_id, _enemy, enemy_collider, enemy_position) in (
				&entities,
				&sto_enemy,
				&sto_rectangle_collider,
				&sto_position,
			).join() {
				// Ignore collisions with enemies that have already been hit.
				if slash_attack.has_been_hit(enemy_id) {
					continue;
				}
				let intersecting = rect_intersects_half_disk(
					(&enemy_collider, &enemy_position),
					(&slash_attack_collider, &slash_attack_position),
				);
				if intersecting {
					// Knock back and damage enemy.
					sto_knocked_back.insert(enemy_id, KnockedBack::from_positions(slash_attack_position, enemy_position)).unwrap();
					sto_health.get_mut(enemy_id).unwrap().damage(1);
					// Mark this enemy has having been hit by this attack.
					slash_attack.mark_as_hit(enemy_id);
				}
			}
		}
		// Handle arrow collisions.
		for (arrow_attack_id, arrow_attack, arrow_attack_collider, arrow_attack_position) in (
			&entities,
			&sto_arrow_attack,
			&sto_rectangle_collider,
			&sto_position,
		).join() {
			let mut max_overlap_area = 0.0;
			let mut closest_colliding_target_id_and_position = None;
			for (target_id, target_faction, target_collider, target_position) in (
				&entities,
				&sto_faction,
				&sto_rectangle_collider,
				&sto_position,
			).join() {
				// Ignore collisions with entities from the same faction.
				if arrow_attack.faction() == *target_faction {
					continue;
				}
				let overlap_area = rect_rect_intersection_area(
					(&arrow_attack_collider, &arrow_attack_position),
					(&target_collider, &target_position),
				);
				if overlap_area > max_overlap_area {
					max_overlap_area = overlap_area;
					closest_colliding_target_id_and_position = Some((target_id, target_position));
				}
			}
			// If any targets were close enough for a collision, damage and knock back the closest one.
			if let Some((target_id, target_position)) = closest_colliding_target_id_and_position {
				// Damage and knock back target.
				sto_knocked_back.insert(target_id, KnockedBack::from_positions(arrow_attack_position, &target_position)).unwrap();
				sto_health.get_mut(target_id).unwrap().damage(1);
				// Destroy the arrow attack.
				entities.delete(arrow_attack_id).unwrap();
			}
		}
		// Handle hero-enemy collisions.
		let mut max_overlap_area = 0.0;
		let mut closest_colliding_position = None;
		for (hero_id, _hero, hero_collider, hero_position, hero_health) in (
			&entities,
			&sto_hero,
			&sto_rectangle_collider,
			&sto_position,
			&mut sto_health,
		).join() {
			// Ignore invulnerable heroes.
			if sto_invulnerable.contains(hero_id) {
				continue;
			}
			// Find the closest enemy in collision with the hero.
			for (enemy_id, _enemy, enemy_collider, enemy_position) in (
				&entities,
				&sto_enemy,
				&sto_rectangle_collider,
				&sto_position,
			).join() {
				// Ignore collisions with knocked back enemies.
				if sto_knocked_back.contains(enemy_id) {
					continue;
				}
				let overlap_area = rect_rect_intersection_area(
					(&hero_collider, &hero_position),
					(&enemy_collider, &enemy_position),
				);
				if overlap_area > max_overlap_area {
					max_overlap_area = overlap_area;
					closest_colliding_position = Some(enemy_position);
				}
			}
			// If any enemies were close enough for a collision, knock the hero back from the closest one.
			if let Some(enemy_position) = closest_colliding_position {
				// Damage and knock back hero.
				sto_knocked_back.insert(hero_id, KnockedBack::from_positions(enemy_position, hero_position)).unwrap();
				hero_health.damage(1);
				// Give hero invulnerability.
				sto_invulnerable.insert(hero_id, Invulnerable::new()).unwrap();
			}
		}
		// Handle hero-heart collisions.
		for (_hero, hero_collider, hero_position, hero_health) in (
			&sto_hero,
			&sto_rectangle_collider,
			&sto_position,
			&mut sto_health,
		).join() {
			// For each heart in contact with the hero, destroy the heart and increase the hero's health.
			for (heart_id, _heart, heart_collider, heart_position) in (
				&entities,
				&sto_heart,
				&sto_rectangle_collider,
				&sto_position,
			).join() {
				let overlap_area = rect_rect_intersection_area(
					(&hero_collider, &hero_position),
					(&heart_collider, &heart_position),
				);
				if overlap_area > 0.0 {
					entities.delete(heart_id).unwrap();
					hero_health.heal(1);
				}
			}
		}
	}
}
