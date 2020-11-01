use crate::{
	component::{
		Collider,
		Enemy,
		Health,
		Hero,
		KnockedBack,
		Position,
		SwordAttack,
		Velocity,
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
		ReadStorage<'a, Collider>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, Hero>,
		ReadStorage<'a, Enemy>,
		WriteStorage<'a, SwordAttack>,
		WriteStorage<'a, Health>,
		WriteStorage<'a, KnockedBack>,
	);

	fn run(&mut self, (
		entities,
		sto_collider,
		sto_position,
		sto_hero,
		sto_enemy,
		mut sto_sword_attack,
		mut sto_health,
		mut sto_knocked_back,
	): Self::SystemData) {
		// Handle sword-enemy collisions.
		let mut max_overlap_area = 0.0;
		let mut closest_colliding_enemy_id_and_position = None;
		let sword_components_iter = (
			&mut sto_sword_attack,
			&sto_collider,
			&sto_position,
		).join();
		for (sword_attack, sword_collider, sword_position) in sword_components_iter {
			// Ignore sword attack collisions if sword is inactive.
			if !sword_attack.is_active() {
				continue;
			}
			// Find the closest enemy in collision with the sword attack.
			let enemy_component_iter = (&entities, &sto_enemy, &sto_collider, &sto_position).join();
			for (enemy_id, _enemy, enemy_collider, enemy_position) in enemy_component_iter {
				// Ignore collisions with knocked back enemies.
				if sto_knocked_back.contains(enemy_id) {
					continue;
				}
				let min_right = (sword_position.x + sword_collider.half_width).min(enemy_position.x + enemy_collider.half_width);
				let max_left = (sword_position.x - sword_collider.half_width).max(enemy_position.x - enemy_collider.half_width);
				let min_top = (sword_position.y + sword_collider.half_height).min(enemy_position.y + enemy_collider.half_height);
				let max_bottom = (sword_position.y - sword_collider.half_height).max(enemy_position.y - enemy_collider.half_height);
				let overlap_area = (min_right - max_left).max(0.0) * (min_top - max_bottom).max(0.0);
				if overlap_area > max_overlap_area {
					max_overlap_area = overlap_area;
					closest_colliding_enemy_id_and_position = Some((enemy_id, enemy_position));
				}
			}
			// If any enemies were close enough for a collision, knock the closest one back.
			if let Some((enemy_id, enemy_position)) = closest_colliding_enemy_id_and_position {
				const KNOCKBACK_SPEED: f32 = 12.5;
				const KNOCKBACK_FRAMES: u32 = 5;
				// Compute heading of velocity based on displacement from the sword to the enemy.
				let mut velocity = Velocity {
					x: enemy_position.x - sword_position.x,
					y: enemy_position.y - sword_position.y,
				};
				// Normalize knockback to a set speed.
				if velocity.x != 0.0 || velocity.y != 0.0 {
					let magnitude = f32::sqrt(velocity.x.powi(2) + velocity.y.powi(2));
					velocity.x *= KNOCKBACK_SPEED / magnitude;
					velocity.y *= KNOCKBACK_SPEED / magnitude;
				} else {
					// The sword is directly on top of the enemy. Knock towards the right arbitrarily.
					velocity.x = KNOCKBACK_SPEED;
				}
				sto_knocked_back.insert(enemy_id, KnockedBack {
					frames_left: KNOCKBACK_FRAMES,
					velocity,
				}).unwrap();
				// Damage enemy.
				sto_health.get_mut(enemy_id).unwrap().damage(1);
				// Make the sword attack inactive now that it has hit an enemy.
				sword_attack.make_inactive();
			}
		}
		// Handle hero-enemy collisions.
		let mut max_overlap_area = 0.0;
		let mut closest_colliding_position = None;
		let hero_components_iter = (
			&entities,
			&sto_hero,
			&sto_collider,
			&sto_position,
			&mut sto_health,
		).join();
		for (hero_id, _hero, hero_collider, hero_position, hero_health) in hero_components_iter {
			// Ignore enemy collisions if already knocked back.
			if sto_knocked_back.contains(hero_id) {
				continue;
			}
			// Find the closest enemy in collision with the hero.
			let enemy_components_iter = (&entities, &sto_enemy, &sto_collider, &sto_position).join();
			for (enemy_id, _enemy, enemy_collider, enemy_position) in enemy_components_iter {
				// Ignore collisions with knocked back enemies.
				if sto_knocked_back.contains(enemy_id) {
					continue;
				}
				let min_right = (hero_position.x + hero_collider.half_width).min(enemy_position.x + enemy_collider.half_width);
				let max_left = (hero_position.x - hero_collider.half_width).max(enemy_position.x - enemy_collider.half_width);
				let min_top = (hero_position.y + hero_collider.half_height).min(enemy_position.y + enemy_collider.half_height);
				let max_bottom = (hero_position.y - hero_collider.half_height).max(enemy_position.y - enemy_collider.half_height);
				let overlap_area = (min_right - max_left).max(0.0) * (min_top - max_bottom).max(0.0);
				if overlap_area > max_overlap_area {
					max_overlap_area = overlap_area;
					closest_colliding_position = Some(enemy_position);
				}
			}
			// If any enemies were close enough for a collision, knock the hero back from the closest one.
			if let Some(enemy_position) = closest_colliding_position {
				const KNOCKBACK_SPEED: f32 = 12.5;
				const KNOCKBACK_FRAMES: u32 = 5;
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
	}
}
