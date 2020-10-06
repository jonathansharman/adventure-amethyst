use crate::{
	component::{Direction, Hero, Position},
	input_bindings::{InputBindings, Actions},
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
	input::InputHandler,
	shred::{Read},
};

use log::info;

use std::convert::TryFrom;

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct HeroControl;

impl<'a> System<'a> for HeroControl {
	type SystemData = (
		Read<'a, InputHandler<InputBindings>>,
		ReadStorage<'a, Hero>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Position>,
	);

	fn run(&mut self, (input, all_heroes, mut all_directions, mut all_positions): Self::SystemData) {
		const ORTHOGONAL_SPEED: f32 = 5.0;
		const DIAGONAL_SPEED: f32 = 0.70710678118 * 5.0;
		for (_hero, direction, position) in (&all_heroes, &mut all_directions, &mut all_positions).join() {
			// Move.
			let mut vx: i16 = 0;
			let mut vy: i16 = 0;
			if input.action_is_down(&Actions::Up).unwrap() {
				vy += 1;
			}
			if input.action_is_down(&Actions::Down).unwrap() {
				vy -= 1;
			}
			if input.action_is_down(&Actions::Left).unwrap() {
				vx -= 1;
			}
			if input.action_is_down(&Actions::Right).unwrap() {
				vx += 1;
			}
			// Update direction.
			match (vx, vy) {
				(0, -1) => *direction = Direction::Down,
				(0, 1) => *direction = Direction::Up,
				(-1, 0) => *direction = Direction::Left,
				(1, 0) => *direction = Direction::Right,
				_ => {},
			};
			// Update translation.
			if vx == 0 || vy == 0 {
				position.x += f32::try_from(vx).unwrap() * ORTHOGONAL_SPEED;
				position.y += f32::try_from(vy).unwrap() * ORTHOGONAL_SPEED;
			} else {
				position.x += f32::try_from(vx).unwrap() * DIAGONAL_SPEED;
				position.y += f32::try_from(vy).unwrap() * DIAGONAL_SPEED;
			}

			// Attack.
			if input.action_is_down(&Actions::Primary).unwrap() {
				info!("Stab!\n");
			}
		}
	}
}
