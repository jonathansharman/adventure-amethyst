use crate::component::{
	behavior::Wander,
	Direction,
	KnockedBack,
	Velocity,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};
use rand::Rng;

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct EnemyControl;

impl<'a> System<'a> for EnemyControl {
	type SystemData = (
		ReadStorage<'a, Wander>,
		ReadStorage<'a, KnockedBack>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Velocity>,
	);

	fn run(&mut self, (all_wanders, all_knocked_backs, mut all_directions, mut all_velocities): Self::SystemData) {
		const SPEED: f32 = 3.0;
		const TURN_THRESHOLD: f32 = 0.01;
		const STOP_THRESHOLD: f32 = TURN_THRESHOLD + 0.01;

		let mut rng = rand::thread_rng();
		let component_iter = (
			&all_wanders,
			!&all_knocked_backs,
			&mut all_directions,
			&mut all_velocities,
		).join();
		for (_wander, _knocked_back, direction, velocity) in component_iter {
			// Randomly change direction or stop.
			let p = rng.gen::<f32>();
			if (0.0..TURN_THRESHOLD).contains(&p) {
				*direction = rng.gen();
				*velocity = match *direction {
					Direction::Up => Velocity { x: 0.0, y: SPEED },
					Direction::Down => Velocity { x: 0.0, y: -SPEED },
					Direction::Left => Velocity { x: -SPEED, y: 0.0 },
					Direction::Right => Velocity { x: SPEED, y: 0.0 },
				}
			} else if (TURN_THRESHOLD..STOP_THRESHOLD).contains(&p) {
				*velocity = Velocity::default();
			}
		}
	}
}
