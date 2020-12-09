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
			let (low, high) = get_low_high(&collider, &position);

			// Compute how far each collider edge extends into its containing tile.
			let tile_center = |position: Position| {
				Position {
					x: TILE_SIZE * (position.x / TILE_SIZE + 0.5).floor(),
					y: TILE_SIZE * (position.y / TILE_SIZE - 0.5).ceil(),
				}
			};
			let low_center = tile_center(low);
			let high_center = tile_center(high);
			let left_extent = low_center.x + 0.5 * TILE_SIZE - low.x;
			let right_extent = high.x - (high_center.x - 0.5 * TILE_SIZE);
			let bottom_extent = low_center.y + 0.5 * TILE_SIZE - low.y;
			let top_extent = high.y - (high_center.y - 0.5 * TILE_SIZE);

			// Check for collisions with walls on each corner.
			let bottom_left = is_wall(low.x, low.y);
			let bottom_right = is_wall(high.x, low.y);
			let top_left = is_wall(low.x, high.y);
			let top_right = is_wall(high.x, high.y);

			// Check for collisions with walls along each side, between corners.
			let x_steps = ((high.x - low.x) / TILE_SIZE).ceil() as i32;
			let y_steps = ((high.y - low.y) / TILE_SIZE).ceil() as i32;
			let bottom_side = (1..x_steps).any(|x_step| {
				let x = (low.x + (x_step as f32 * TILE_SIZE)).min(high.x);
				is_wall(x, low.y)
			});
			let top_side = (1..x_steps).any(|x_step| {
				let x = (low.x + (x_step as f32 * TILE_SIZE)).min(high.x);
				is_wall(x, high.y)
			});
			let left_side = (1..y_steps).any(|y_step| {
				let y = (low.y + (y_step as f32 * TILE_SIZE)).min(high.y);
				is_wall(low.x, y)
			});
			let right_side = (1..y_steps).any(|y_step| {
				let y = (low.y + (y_step as f32 * TILE_SIZE)).min(high.y);
				is_wall(high.x, y)
			});

			// Compute the "weights" of the four collision sides, which determines the certainty that
			// the collider should be pushed away from that side.
			//
			// Side collisions are weighted twice as much as corner collisions. This accounts for the
			// fact that a corner collision can contribute to a push in two different directions, but
			// a side collision always pushes the same way and so adds more certainty to that direction.
			let bottom_weight = bottom_left as i32 + bottom_side as i32 * 2 + bottom_right as i32;
			let top_weight = top_left as i32 + top_side as i32 * 2 + top_right as i32;
			let left_weight = bottom_left as i32 + left_side as i32 * 2 + top_left as i32;
			let right_weight = bottom_right as i32 + right_side as i32 * 2 + top_right as i32;

			// Check for overlap only of a single corner, in which case the total weight is 2 (1 for each adjacent side).
			let at_least_one_corner = bottom_left || bottom_right || top_left || top_right;
			let total_weight = bottom_weight + top_weight + left_weight + right_weight;
			if at_least_one_corner && total_weight == 2 {
				// In this case, need to find the minimum distance needed to push the collider out of the wall.
				let mut push_direction: Option<Direction> = None;
				let mut min_push: f32 = f32::INFINITY;
				if bottom_left {
					// Bottom collision
					min_push = bottom_extent;
					push_direction = Some(Direction::Up);
					// Left collision
					if left_extent < min_push {
						min_push = left_extent;
						push_direction = Some(Direction::Right);
					}
				} else if bottom_right {
					// Bottom collision
					if bottom_extent < min_push {
						min_push = bottom_extent;
						push_direction = Some(Direction::Up);
					}
					// Right collision
					if right_extent < min_push {
						min_push = right_extent;
						push_direction = Some(Direction::Left);
					}
				} else if top_left {
					// Top collision
					if top_extent < min_push {
						min_push = top_extent;
						push_direction = Some(Direction::Down);
					}
					// Left collision
					if left_extent < min_push {
						min_push = left_extent;
						push_direction = Some(Direction::Right);
					}
				} else if top_right {
					// Top collision
					if top_extent < min_push {
						min_push = top_extent;
						push_direction = Some(Direction::Down);
					}
					// Right collision
					if right_extent < min_push {
						min_push = right_extent;
						push_direction = Some(Direction::Left);
					}
				}
				// If there was a single corner hit, perform the push.
				if let Some(push_direction) = push_direction {
					match push_direction {
						Direction::Up => position.y += min_push,
						Direction::Down => position.y -= min_push,
						Direction::Right => position.x += min_push,
						Direction::Left => position.x -= min_push,
					};
				}
			} else {
				// In this case, there are side hits or multiple corner hits. Require that there be more "weight"
				// on a given side than on its opposite side in order to push away from it. Furthermore, require
				// at least two corner hits or one side hit in order to push because pushing when just one corner
				// overlaps results in false positives, and the single corner case is handled above.
				if bottom_weight > top_weight && bottom_weight >= 2 {
					position.y += bottom_extent;
				} else if top_weight > bottom_weight && top_weight >= 2 {
					position.y -= top_extent;
				}
				if left_weight > right_weight && left_weight >= 2 {
					position.x += left_extent;
				} else if right_weight > left_weight && right_weight >= 2 {
					position.x -= right_extent;
				}
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
			let x_steps = ((high.x - low.x) / TILE_SIZE).ceil() as i32 + 1;
			let y_steps = ((high.y - low.y) / TILE_SIZE).ceil() as i32 + 1;
			for x_step in 0..x_steps {
				let x = low.x + (x_step as f32 * TILE_SIZE).min(high.x - low.x);
				for y_step in 0..y_steps {
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
