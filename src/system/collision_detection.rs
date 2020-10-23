use crate::{
	component::{
		Collider,
		Direction,
		Enemy,
		Hero,
		KnockedBack,
		Position,
		Terrain,
		Velocity,
	},
	constants::*,
	resource::Region,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

/// Does collision detection and response for characters.
#[derive(SystemDesc)]
pub struct CollisionDetection;

impl<'a> System<'a> for CollisionDetection {
	type SystemData = (
		ReadExpect<'a, Region>,
		Entities<'a>,
		ReadStorage<'a, Collider>,
		ReadStorage<'a, Terrain>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, Hero>,
		ReadStorage<'a, Enemy>,
		WriteStorage<'a, KnockedBack>,
	);

	fn run(&mut self, (
		region,
		entities,
		all_colliders,
		all_terrains,
		mut all_positions,
		all_heroes,
		all_enemies,
		mut all_knocked_backs,
	): Self::SystemData) {
		// Push colliders out of obstacles.
		for (collider, position) in (&all_colliders, &mut all_positions).join() {
			if collider.width > TILE_SIZE || collider.height > TILE_SIZE {
				panic!("Collider with width or height larger than tile width/height not supported");
			}

			// The nearest four-tile corner.
			let snap = Position {
				x: TILE_SIZE * ((position.x / TILE_SIZE).floor() + 0.5),
				y: TILE_SIZE * ((position.y / TILE_SIZE).ceil() - 0.5),
			};
			// The position of the collider's bottom-left corner.
			let low = Position {
				x: position.x - collider.width / 2.0,
				y: position.y - collider.height / 2.0,
			};
			// The position of the collider's upper-right corner.
			let high = Position {
				x: position.x + collider.width / 2.0,
				y: position.y + collider.height / 2.0,
			};

			// Detect collisions with the surrounding walls in each diagonal direction.
			let is_wall = |x, y| {
				region.terrain_at_position(&all_terrains, Position { x, y })
					.map_or(false, |terrain| terrain.blocks_movement())
			};
			let bottom_left = is_wall(low.x, low.y);
			let bottom_right = is_wall(high.x, low.y);
			let top_left = is_wall(low.x, high.y);
			let top_right = is_wall(high.x, high.y);
			//log::info!("{} {} {} {}", bottom_left, bottom_right, top_left, top_right);

			// Easy cases: hitting at least two walls at once.
			let mut multi_hit = false;
			if bottom_left && bottom_right {
				// Bottom collision
				position.y = snap.y + collider.height / 2.0;
				multi_hit = true;
			} else if top_left && top_right {
				// Top collision
				position.y = snap.y - collider.height / 2.0;
				multi_hit = true;
			}
			if bottom_left && top_left {
				// Left collision
				position.x = snap.x + collider.width / 2.0;
				multi_hit = true;
			} else if bottom_right && top_right {
				// Right collision
				position.x = snap.x - collider.width / 2.0;
				multi_hit = true;
			}
			if multi_hit {
				continue;
			}

			// Harder case: hitting just one wall. Need to find the minimum distance needed to push the collider out of the wall.
			let mut push_direction: Option<Direction> = None;
			let mut min_push: f32 = f32::INFINITY;

			if bottom_left {
				// Bottom collision
				min_push = snap.y - low.y;
				push_direction = Some(Direction::Up);
				// Left collision
				let push = snap.x - low.x;
				if push < min_push {
					min_push = push;
					push_direction = Some(Direction::Right);
				}
			}
			if bottom_right {
				// Bottom collision
				let push = snap.y - low.y;
				if push < min_push {
					min_push = push;
					push_direction = Some(Direction::Up);
				}
				// Right collision
				let push = high.x - snap.x;
				if push < min_push {
					min_push = push;
					push_direction = Some(Direction::Left);
				}
			}
			if top_left {
				// Top collision
				let push = high.y - snap.y;
				if push < min_push {
					min_push = push;
					push_direction = Some(Direction::Down);
				}
				// Left collision
				let push = snap.x - low.x;
				if push < min_push {
					min_push = push;
					push_direction = Some(Direction::Right);
				}
			}
			if top_right {
				// Top collision
				let push = high.y - snap.y;
				if push < min_push {
					min_push = push;
					push_direction = Some(Direction::Down);
				}
				// Right collision
				let push = high.x - snap.x; 
				if push < min_push {
					push_direction = Some(Direction::Left);
				}
			}

			// Perform the push.
			if let Some(push_direction) = push_direction {
				match push_direction {
					Direction::Up => position.y = snap.y + collider.height / 2.0,
					Direction::Down => position.y = snap.y - collider.height / 2.0,
					Direction::Right => position.x = snap.x + collider.width / 2.0,
					Direction::Left => position.x = snap.x - collider.width / 2.0,
				};
			}
		}

		// Handle hero-enemy collisions.
		let mut max_overlap_area = 0.0;
		let mut closest_colliding_position = None;
		let hero_component_iter = (&entities, &all_heroes, &all_colliders, &all_positions).join();
		for (hero_entity, _hero, hero_collider, hero_position) in hero_component_iter {
			if all_knocked_backs.contains(hero_entity) {
				// Ignore enemy collisions while already knocked back.
				continue;
			}
			// Find the closest enemy in collision with the hero.
			let enemy_component_iter = (&all_enemies, &all_colliders, &all_positions).join();
			for (_enemy, enemy_collider, enemy_position) in enemy_component_iter {
				let min_right = (hero_position.x + hero_collider.width / 2.0).min(enemy_position.x + enemy_collider.width / 2.0);
				let max_left = (hero_position.x - hero_collider.width / 2.0).max(enemy_position.x - enemy_collider.width / 2.0);
				let min_top = (hero_position.y + hero_collider.height / 2.0).min(enemy_position.y + enemy_collider.height / 2.0);
				let max_bottom = (hero_position.y - hero_collider.height / 2.0).max(enemy_position.y - enemy_collider.height / 2.0);
				let overlap_area = (min_right - max_left).max(0.0) * (min_top - max_bottom).max(0.0);
				if overlap_area > max_overlap_area {
					max_overlap_area = overlap_area;
					closest_colliding_position = Some(enemy_position);
				}
			}
			// If any enemies were close enough for a collision, knock the hero back from the closest one.
			if let Some(enemy_position) = closest_colliding_position {
				const KNOCKBACK_SPEED: f32 = 10.0;
				const KNOCKBACK_FRAMES: u32 = 10;
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
				all_knocked_backs.insert(hero_entity, KnockedBack {
					frames_left: KNOCKBACK_FRAMES,
					velocity,
				}).unwrap();
			}
		}
	}
}
