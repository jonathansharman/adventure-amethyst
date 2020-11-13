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
		WriteStorage<'a, Wander>,
		ReadStorage<'a, KnockedBack>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Velocity>,
	);

	fn run(&mut self, (mut sto_wander, sto_knocked_back, mut sto_direction, mut sto_velocity): Self::SystemData) {
		const SPEED: f32 = 3.0;
		const TURN_THRESHOLD: f32 = 0.01;
		const STOP_THRESHOLD: f32 = TURN_THRESHOLD + 0.01;

		let mut rng = rand::thread_rng();
		for (wander, _no_knocked_back, direction, velocity) in (
			&mut sto_wander,
			// No control while being knocked back
			!&sto_knocked_back,
			&mut sto_direction,
			&mut sto_velocity,
		).join() {
			// Randomly change wander direction or stop.
			let p = rng.gen::<f32>();
			if (0.0..TURN_THRESHOLD).contains(&p) {
				wander.direction = Some(rng.gen());
			} else if (TURN_THRESHOLD..STOP_THRESHOLD).contains(&p) {
				wander.direction = None;
			}
			// Move according to current wander direction.
			match wander.direction {
				Some(wander_direction) => {
					*direction = wander_direction;
					*velocity = match wander_direction {
						Direction::Up => Velocity { x: 0.0, y: SPEED },
						Direction::Down => Velocity { x: 0.0, y: -SPEED },
						Direction::Left => Velocity { x: -SPEED, y: 0.0 },
						Direction::Right => Velocity { x: SPEED, y: 0.0 },
					}
				},
				None => *velocity = Velocity::default(),
			}
		}
	}
}
