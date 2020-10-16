use crate::component::{
	Position,
	Velocity,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Join, System, SystemData, WriteStorage},
};

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct Motion;

impl<'a> System<'a> for Motion {
	type SystemData = (
		WriteStorage<'a, Position>,
		WriteStorage<'a, Velocity>,
	);

	fn run(&mut self, (mut all_positions, mut all_velocities): Self::SystemData) {
		for (position, velocity) in (&mut all_positions, &mut all_velocities).join() {
			position.x += velocity.x;
			position.y += velocity.y;
		}
	}
}
