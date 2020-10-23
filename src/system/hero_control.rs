use crate::{
	component::{
		Direction,
		Hero,
		KnockedBack,
		Velocity,
	},
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
		ReadStorage<'a, KnockedBack>,
		WriteStorage<'a, Velocity>,
		WriteStorage<'a, Direction>,
	);

	fn run(&mut self, (input, all_heroes, all_knock_backs, mut all_velocities, mut all_directions): Self::SystemData) {
		const ORTHOGONAL_SPEED: f32 = 5.0;
		const DIAGONAL_SPEED: f32 = 0.70710678118 * ORTHOGONAL_SPEED;
		let component_iter = (&all_heroes, !&all_knock_backs, &mut all_velocities, &mut all_directions).join();
		for (_hero, _knock_back, velocity, direction) in component_iter {
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
			// Update direction if needed. There are eight directions of movement but only four for animation.
			match (*direction, vx, vy) {
				// Moving in a cardinal direction.
				(_, 0, -1) => *direction = Direction::Down,
				(_, 0, 1) => *direction = Direction::Up,
				(_, -1, 0) => *direction = Direction::Left,
				(_, 1, 0) => *direction = Direction::Right,
				// Facing up but moving down-left, down, or right.
				(Direction::Up, _, -1) => *direction = Direction::Down,
				// Facing down but moving up-left, up, or right.
				(Direction::Down, _, 1) => *direction = Direction::Up,
				// Facing right but moving up-left, left, or down-left.
				(Direction::Right, -1, _) => *direction = Direction::Left,
				// Facing left but moving up-right, right, or down-right.
				(Direction::Left, 1, _) => *direction = Direction::Right,
				// Already facing in a reasonable direction.
				_ => {},
			};
			// Update translation.
			if vx == 0 || vy == 0 {
				velocity.x = f32::try_from(vx).unwrap() * ORTHOGONAL_SPEED;
				velocity.y = f32::try_from(vy).unwrap() * ORTHOGONAL_SPEED;
			} else {
				velocity.x = f32::try_from(vx).unwrap() * DIAGONAL_SPEED;
				velocity.y = f32::try_from(vy).unwrap() * DIAGONAL_SPEED;
			}

			// Attack.
			if input.action_is_down(&Actions::Primary).unwrap() {
				info!("Stab!\n");
			}
		}
	}
}
