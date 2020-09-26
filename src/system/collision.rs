use crate::{
	component::{Collider, Terrain},
	constants::*,
	resource::Region,
};

use amethyst::{
	core::Transform,
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
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (region, colliders, terrains, mut transforms): Self::SystemData) {
		for (_, transform) in (&colliders, &mut transforms).join() {
			let translation = transform.translation();
			let x = translation.x;
			let x_floor = TILE_SIZE * (x / TILE_SIZE).floor();
			let y = translation.y;
			let y_floor = TILE_SIZE * (y / TILE_SIZE).floor();

			// Detect collisions with the surrounding walls in each diagonal direction.
			let is_wall = |x, y| region.terrain_at_x_y(&terrains, x, y) == Some(Terrain::Wall);
			let bottom_left = is_wall(x_floor, y_floor);
			let bottom_right = is_wall(x_floor + TILE_SIZE, y_floor);
			let top_left = is_wall(x_floor, y_floor + TILE_SIZE);
			let top_right = is_wall(x_floor + TILE_SIZE, y_floor + TILE_SIZE);

			// Easy cases: hitting at least two walls at once.
			let mut multi_hit = false;
			if bottom_left && bottom_right {
				// Bottom collision
				transform.set_translation_y(y_floor + TILE_SIZE);
				multi_hit = true;
			} else if top_left && top_right {
				// Top collision
				transform.set_translation_y(y_floor);
				multi_hit = true;
			}
			if bottom_left && top_left {
				// Left collision
				transform.set_translation_x(x_floor + TILE_SIZE);
				multi_hit = true;
			} else if bottom_right && top_right {
				// Right collision
				transform.set_translation_x(x_floor);
				multi_hit = true;
			}
			if multi_hit {
				return;
			}

			// Harder case: hitting just one wall. Need to find the minimum distance needed to push the collider out of the wall.
			enum PushDirection { Up, Down, Right, Left }
			let mut push_direction: Option<PushDirection> = None;
			let mut min_push: f32 = f32::INFINITY;

			if bottom_left {
				// Bottom collision
				min_push = TILE_SIZE - (y - y_floor);
				push_direction = Some(PushDirection::Up);
				// Left collision
				let push = TILE_SIZE - (x - x_floor);
				if push < min_push {
					min_push = push;
					push_direction = Some(PushDirection::Right);
				}
			}
			if bottom_right {
				// Bottom collision
				let push = TILE_SIZE - (y - y_floor);
				if push < min_push {
					min_push = push;
					push_direction = Some(PushDirection::Up);
				}
				// Right collision
				let push = x - x_floor;
				if push < min_push {
					min_push = push;
					push_direction = Some(PushDirection::Left);
				}
			}
			if top_left {
				// Top collision
				let push = y - y_floor;
				if push < min_push {
					min_push = push;
					push_direction = Some(PushDirection::Down);
				}
				// Left collision
				let push = TILE_SIZE - (x - x_floor);
				if push < min_push {
					min_push = push;
					push_direction = Some(PushDirection::Right);
				}
			}
			if top_right {
				// Top collision
				let push = y - y_floor;
				if push < min_push {
					min_push = push;
					push_direction = Some(PushDirection::Down);
				}
				// Right collision
				let push = x - x_floor;
				if push < min_push {
					push_direction = Some(PushDirection::Left);
				}
			}

			// Perform the push.
			if let Some(push_direction) = push_direction {
				match push_direction {
					PushDirection::Up => transform.set_translation_y(y_floor + TILE_SIZE),
					PushDirection::Down => transform.set_translation_y(y_floor),
					PushDirection::Right => transform.set_translation_x(x_floor + TILE_SIZE),
					PushDirection::Left => transform.set_translation_x(x_floor),
				};
			}
		}
	}
}