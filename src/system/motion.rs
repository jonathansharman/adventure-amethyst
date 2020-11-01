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

	fn run(&mut self, (mut sto_position, mut sto_velocity): Self::SystemData) {
		for (position, velocity) in (&mut sto_position, &mut sto_velocity).join() {
			position.x += velocity.x;
			position.y += velocity.y;
		}
	}
}
