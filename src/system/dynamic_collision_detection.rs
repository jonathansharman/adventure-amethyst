use crate::{
	component::{
		Collider,
		Enemy,
		Hero,
		KnockedBack,
		Position,
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
		WriteStorage<'a, KnockedBack>,
	);

	fn run(&mut self, (
		entities,
		all_colliders,
		all_positions,
		all_heroes,
		all_enemies,
		mut all_knocked_backs,
	): Self::SystemData) {
		// Handle hero-enemy collisions.
		let mut max_overlap_area = 0.0;
		let mut closest_colliding_position = None;
		let hero_component_iter = (&entities, &all_heroes, &all_colliders, &all_positions).join();
		for (hero_id, _hero, hero_collider, hero_position) in hero_component_iter {
			if all_knocked_backs.contains(hero_id) {
				// Ignore enemy collisions while already knocked back.
				continue;
			}
			// Find the closest enemy in collision with the hero.
			let enemy_component_iter = (&all_enemies, &all_colliders, &all_positions).join();
			for (_enemy, enemy_collider, enemy_position) in enemy_component_iter {
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
				all_knocked_backs.insert(hero_id, KnockedBack {
					frames_left: KNOCKBACK_FRAMES,
					velocity,
				}).unwrap();
			}
		}
	}
}
