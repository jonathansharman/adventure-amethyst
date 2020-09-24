use crate::{
	component::{Direction, Hero},
	input_bindings::{InputBindings, Actions},
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
	input::InputHandler,
	renderer::SpriteRender,
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
		WriteStorage<'a, Transform>,
		WriteStorage<'a, SpriteRender>,
	);

	fn run(&mut self, (input, heroes, mut directions, mut transforms, mut sprites): Self::SystemData) {
		const ORTHOGONAL_SPEED: f32 = 5.0;
		const DIAGONAL_SPEED: f32 = 0.70710678118 * 5.0;
		for (_, direction, transform, sprite) in (&heroes, &mut directions, &mut transforms, &mut sprites).join() {
			// Move.
			let mut vx: i16 = 0;
			let mut vy: i16 = 0;
			if input.action_is_down(&Actions::Up).unwrap() {
				vy -= 1;
			}
			if input.action_is_down(&Actions::Down).unwrap() {
				vy += 1;
			}
			if input.action_is_down(&Actions::Left).unwrap() {
				vx -= 1;
			}
			if input.action_is_down(&Actions::Right).unwrap() {
				vx += 1;
			}
			// Update direction.
			match (vx, vy) {
				(0, -1) => {
					*direction = Direction::Up;
					sprite.sprite_number = 0;
				},
				(0, 1) => {
					*direction = Direction::Down;
					sprite.sprite_number = 1;
				},
				(-1, 0) => {
					*direction = Direction::Left;
					sprite.sprite_number = 2;
				},
				(1, 0) => {
					*direction = Direction::Right;
					sprite.sprite_number = 3;
				},
				_ => {},
			}
			// Update translation.
			if vx == 0 || vy == 0 {
				transform.move_right(f32::try_from(vx).unwrap() * ORTHOGONAL_SPEED);
				transform.move_down(f32::try_from(vy).unwrap() * ORTHOGONAL_SPEED);
			} else {
				transform.move_right(f32::try_from(vx).unwrap() * DIAGONAL_SPEED);
				transform.move_down(f32::try_from(vy).unwrap() * DIAGONAL_SPEED);
			}

			// Attack.
			if input.action_is_down(&Actions::Primary).unwrap() {
				info!("Stab!\n");
			}
		}
	}
}
