use crate::component::{
	behavior::Wander,
	Direction,
	Position,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Join, System, SystemData, WriteStorage},
};
use rand::Rng;

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct EnemyControl;

impl<'a> System<'a> for EnemyControl {
	type SystemData = (
		WriteStorage<'a, Wander>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Direction>,
	);

	fn run(&mut self, (mut all_wanders, mut all_positions, mut all_directions): Self::SystemData) {
		const SPEED: f32 = 3.0;
		const TURN_THRESHOLD: f32 = 0.01;
		const STOP_THRESHOLD: f32 = TURN_THRESHOLD + 0.01;

		let mut rng = rand::thread_rng();
		let component_iter = (&mut all_wanders, &mut all_positions, &mut all_directions).join();
		for (wander, position, direction) in component_iter {
			// Randomly change direction sometimes.
			let p = rng.gen::<f32>();
			if (0.0..TURN_THRESHOLD).contains(&p) {
				wander.direction = Some(rng.gen());
			} else if (TURN_THRESHOLD..STOP_THRESHOLD).contains(&p) {
				wander.direction = None
			}
			// Move according to current wander direction.
			match wander.direction {
				Some(Direction::Up) => {
					*direction = Direction::Up;
					position.y += SPEED;
				},
				Some(Direction::Down) => {
					*direction = Direction::Down;
					position.y -= SPEED;
				},
				Some(Direction::Left) => {
					*direction = Direction::Left;
					position.x -= SPEED;
				},
				Some(Direction::Right) => {
					*direction = Direction::Right;
					position.x += SPEED;
				},
				None => {},
			}
		}
	}
}
