use crate::{
	component::{
		ArrowAttack,
		collider::RectangleCollider,
		Direction,
		Faction,
		Position,
		Terrain,
	},
	constants::*,
	resource::CurrentRegion,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

/// Does collision detection and response for characters.
#[derive(SystemDesc)]
pub struct StaticCollisionDetection;

impl<'a> System<'a> for StaticCollisionDetection {
	type SystemData = (
		ReadExpect<'a, CurrentRegion>,
		Entities<'a>,
		ReadStorage<'a, Faction>,
		ReadStorage<'a, RectangleCollider>,
		ReadStorage<'a, Terrain>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, ArrowAttack>,
	);

	fn run(&mut self, (
		current_region,
		entities,
		sto_faction,
		sto_rectangle_collider,
		sto_terrain,
		mut sto_position,
		sto_arrow_attack,
	): Self::SystemData) {
		// Determines if the tile containing (x, y) is a wall.
		let is_wall = |x, y| {
			current_region.get().terrain_at_position(&sto_terrain, Position { x, y })
				.map_or(false, |terrain| terrain.blocks_movement())
		};
		// Retrieves the bottom-left and upper-right corners of the given collider and position.
		let get_low_high = |collider: &RectangleCollider, position: &Position| {
			let low = Position {
				x: position.x - collider.half_width,
				y: position.y - collider.half_height,
			};
			let high = Position {
				x: position.x + collider.half_width,
				y: position.y + collider.half_height,
			};
			(low, high)
		};
		// Push characters out of obstacles.
		for (_faction, collider, position) in (
			&sto_faction,
			&sto_rectangle_collider,
			&mut sto_position,
		).join() {
			if 2.0 * collider.half_width > TILE_SIZE || 2.0 * collider.half_height > TILE_SIZE {
				panic!("Collider with width or height larger than tile width or height not supported");
			}

			// The nearest four-tile corner.
			let snap = Position {
				x: TILE_SIZE * ((position.x / TILE_SIZE).floor() + 0.5),
				y: TILE_SIZE * ((position.y / TILE_SIZE).ceil() - 0.5),
			};
			let (low, high) = get_low_high(&collider, &position);

			// Detect collisions with the surrounding walls in each diagonal direction.
			let bottom_left = is_wall(low.x, low.y);
			let bottom_right = is_wall(high.x, low.y);
			let top_left = is_wall(low.x, high.y);
			let top_right = is_wall(high.x, high.y);

			// Easy cases: hitting at least two walls at once.
			let mut multi_hit = false;
			if bottom_left && bottom_right {
				// Bottom collision
				position.y = snap.y + collider.half_height;
				multi_hit = true;
			} else if top_left && top_right {
				// Top collision
				position.y = snap.y - collider.half_height;
				multi_hit = true;
			}
			if bottom_left && top_left {
				// Left collision
				position.x = snap.x + collider.half_width;
				multi_hit = true;
			} else if bottom_right && top_right {
				// Right collision
				position.x = snap.x - collider.half_width;
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
					Direction::Up => position.y = snap.y + collider.half_height,
					Direction::Down => position.y = snap.y - collider.half_height,
					Direction::Right => position.x = snap.x + collider.half_width,
					Direction::Left => position.x = snap.x - collider.half_width,
				};
			}
		}
		// Destroy arrows that hit obstacles.
		'arrow_destruction: for (arrow_attack_id, _arrow_attack, collider, position) in (
			&entities,
			&sto_arrow_attack,
			&sto_rectangle_collider,
			&sto_position,
		).join() {
			let (low, high) = get_low_high(&collider, &position);
			let x_steps = ((high.x - low.x) / TILE_SIZE).ceil() as i32;
			let y_steps = ((high.y - low.y) / TILE_SIZE).ceil() as i32;
			for x_step in 0..=x_steps {
				let x = low.x + (x_step as f32 * TILE_SIZE).min(high.x - low.x);
				for y_step in 0..=y_steps {
					let y = low.y + (y_step as f32 * TILE_SIZE).min(high.y - low.y);
					if is_wall(x, y) {
						entities.delete(arrow_attack_id).unwrap();
						continue 'arrow_destruction;
					}
				}
			}
		}
	}
}
