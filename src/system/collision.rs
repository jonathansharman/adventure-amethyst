use crate::{
	component::{Collider, Direction, Position, Terrain},
	constants::*,
	resource::Region,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

/// Does collision detection and response for characters.
#[derive(SystemDesc)]
pub struct Collision;

impl<'a> System<'a> for Collision {
	type SystemData = (
		ReadExpect<'a, Region>,
		ReadStorage<'a, Collider>,
		ReadStorage<'a, Terrain>,
		WriteStorage<'a, Position>,
	);

	fn run(&mut self, (region, all_colliders, all_terrains, mut all_positions): Self::SystemData) {
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
				return;
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
				let push = high.x - snap.x;
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
	}
}
