use crate::{
	component::{
		collider::{
			HalfDiskCollider,
			RectangleCollider,
			rect_intersects_half_disk,
			rect_rect_intersection_area,
		},
		Enemy,
		Health,
		Heart,
		Hero,
		KnockedBack,
		Position,
		SlashAttack,
		ThrustAttack,
		Velocity,
	},
	constants::*,
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
		WriteStorage<'a, SlashAttack>,
		WriteStorage<'a, ThrustAttack>,
		WriteStorage<'a, Health>,
		WriteStorage<'a, Heart>,
		WriteStorage<'a, KnockedBack>,
	);

	fn run(&mut self, (
		entities,
		sto_rectangle_collider,
		sto_half_disk_collider,
		sto_position,
		sto_hero,
		sto_enemy,
		mut sto_slash_attack,
		mut sto_thrust_attack,
		mut sto_health,
		sto_heart,
		mut sto_knocked_back,
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
				// Ignore collisions with knocked back enemies.
				if sto_knocked_back.contains(enemy_id) {
					continue;
				}
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
				// Compute heading of velocity based on displacement from the thrust to the enemy.
				let mut velocity = Velocity {
					x: enemy_position.x - thrust_attack_position.x,
					y: enemy_position.y - thrust_attack_position.y,
				};
				// Normalize knockback to a set speed.
				if velocity.x != 0.0 || velocity.y != 0.0 {
					let magnitude = f32::sqrt(velocity.x.powi(2) + velocity.y.powi(2));
					velocity.x *= KNOCKBACK_SPEED / magnitude;
					velocity.y *= KNOCKBACK_SPEED / magnitude;
				} else {
					// The thrust is directly on top of the enemy. Knock towards the right arbitrarily.
					velocity.x = KNOCKBACK_SPEED;
				}
				sto_knocked_back.insert(enemy_id, KnockedBack {
					frames_left: KNOCKBACK_FRAMES,
					velocity,
				}).unwrap();
				// Damage enemy.
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
			let enemy_component_iter = (&entities, &sto_enemy, &sto_rectangle_collider, &sto_position).join();
			for (enemy_id, _enemy, enemy_collider, enemy_position) in enemy_component_iter {
				// Ignore collisions with knocked back enemies.
				if sto_knocked_back.contains(enemy_id) {
					continue;
				}
				// Ignore collisions with enemies that have already been hit.
				if slash_attack.has_been_hit(enemy_id) {
					continue;
				}
				let intersecting = rect_intersects_half_disk(
					(&enemy_collider, &enemy_position),
					(&slash_attack_collider, &slash_attack_position),
				);
				if intersecting {
					// Mark this enemy has having been hit by this attack.
					slash_attack.mark_as_hit(enemy_id);
					// Compute heading of velocity based on displacement from the slash attack to the enemy.
					let mut velocity = Velocity {
						x: enemy_position.x - slash_attack_position.x,
						y: enemy_position.y - slash_attack_position.y,
					};
					// Normalize knockback to a set speed.
					if velocity.x != 0.0 || velocity.y != 0.0 {
						let magnitude = f32::sqrt(velocity.x.powi(2) + velocity.y.powi(2));
						velocity.x *= KNOCKBACK_SPEED / magnitude;
						velocity.y *= KNOCKBACK_SPEED / magnitude;
					} else {
						// The slash attack is directly on top of the enemy. Knock towards the right arbitrarily.
						velocity.x = KNOCKBACK_SPEED;
					}
					sto_knocked_back.insert(enemy_id, KnockedBack {
						frames_left: KNOCKBACK_FRAMES,
						velocity,
					}).unwrap();
					// Damage enemy.
					sto_health.get_mut(enemy_id).unwrap().damage(1);
				}
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
			// Ignore enemy collisions if already knocked back.
			if sto_knocked_back.contains(hero_id) {
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
				// Compute heading of velocity based on displacement from the enemy to the hero.
				let mut velocity = Velocity {
					x: hero_position.x - enemy_position.x,
					y: hero_position.y - enemy_position.y,
				};
				// Normalize knockback to a set speed.
				if velocity.x != 0.0 || velocity.y != 0.0 {
					let magnitude = f32::sqrt(velocity.x.powi(2) + velocity.y.powi(2));
					velocity.x *= KNOCKBACK_SPEED / magnitude;
					velocity.y *= KNOCKBACK_SPEED / magnitude;
				} else {
					// The enemy is directly on top of the hero. Knock towards the right arbitrarily.
					velocity.x = KNOCKBACK_SPEED;
				}
				sto_knocked_back.insert(hero_id, KnockedBack {
					frames_left: KNOCKBACK_FRAMES,
					velocity,
				}).unwrap();
				// Damage hero.
				hero_health.damage(1);
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
