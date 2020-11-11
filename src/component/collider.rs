/// Components for entitiy collisions.

use crate::component::{Direction, Position};

use amethyst::ecs::{Component, DenseVecStorage};
use nalgebra::clamp;
use serde::Deserialize;

/// Axis-aligned rectangle collider.
#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct RectangleCollider {
	pub half_width: f32,
	pub half_height: f32,
}

impl Component for RectangleCollider {
	type Storage = DenseVecStorage<Self>;
}

/// Axis-aligned half-disk collider.
#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct HalfDiskCollider {
	pub radius: f32,
	/// The direction towards which the half-disk extends.
	pub direction: Direction,
}

impl Component for HalfDiskCollider {
	type Storage = DenseVecStorage<Self>;
}

/// Determines the area of intersection between two rectangle colliders.
pub fn rect_rect_intersection_area(first: (&RectangleCollider, &Position), second: (&RectangleCollider, &Position)) -> f32 {
	let (collider1, pos1) = first;
	let (collider2, pos2) = second;
	let min_right = (pos1.x + collider1.half_width).min(pos2.x + collider2.half_width);
	let max_left = (pos1.x - collider1.half_width).max(pos2.x - collider2.half_width);
	let min_top = (pos1.y + collider1.half_height).min(pos2.y + collider2.half_height);
	let max_bottom = (pos1.y - collider1.half_height).max(pos2.y - collider2.half_height);
	let overlap_area = (min_right - max_left).max(0.0) * (min_top - max_bottom).max(0.0);
	overlap_area
}

/// Determines whether `rectangle` and `half_disk` intersect.
pub fn rect_intersects_half_disk(rectangle: (&RectangleCollider, &Position), half_disk: (&HalfDiskCollider, &Position)) -> bool {
	let (rect_collider, rect_pos) = rectangle;
	let (half_disk_collider, disk_pos) = half_disk;
	// Find the point inside the rectangle that is closest to the disk's center.
	let closest = Position {
		x: clamp(disk_pos.x, rect_pos.x - rect_collider.half_width, rect_pos.x + rect_collider.half_width),
		y: clamp(disk_pos.y, rect_pos.y - rect_collider.half_height, rect_pos.y + rect_collider.half_height),
	};
	// Check whether this closest point is in the opposite half plane from the half disk.
	let in_different_half_planes = match half_disk_collider.direction {
		Direction::Up => rect_pos.y < disk_pos.y,
		Direction::Down => rect_pos.y > disk_pos.y,
		Direction::Left => rect_pos.x > disk_pos.x,
		Direction::Right => rect_pos.x < disk_pos.x,
	};
	if in_different_half_planes {
		return false;
	}
	// Check whether this closest point is within an epsilon of being within the disk.
	// Using an epsilon to compensate for floating-point inaccuracy when the closest point is along the half disk's edge.
	const EPSILON: f32 = f32::EPSILON;
	let distance_squared = (closest.x - disk_pos.x).powi(2) + (closest.y - disk_pos.y).powi(2);
	distance_squared - EPSILON <= half_disk_collider.radius.powi(2)
}
