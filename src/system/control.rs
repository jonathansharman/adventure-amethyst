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

/// Reads player input and updates the game state accordingly.
#[derive(SystemDesc)]
pub struct Control;

impl<'a> System<'a> for Control {
	type SystemData = (
		Read<'a, InputHandler<InputBindings>>,
		ReadStorage<'a, Hero>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Transform>,
		WriteStorage<'a, SpriteRender>,
	);

	fn run(&mut self, (input, hero, mut direction, mut transform, mut sprite): Self::SystemData) {
		const ORTHOGONAL_SPEED: f32 = 7.0;
		const DIAGONAL_SPEED: f32 = 4.94974746831;
		for (_, direction, transform, sprite) in (&hero, &mut direction, &mut transform, &mut sprite).join() {
			// Move.
			let mut vx = 0;
			let mut vy = 0;
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
				transform.move_right(vx as f32 * ORTHOGONAL_SPEED);
				transform.move_down(vy as f32 * ORTHOGONAL_SPEED);
			} else {
				transform.move_right(vx as f32 * DIAGONAL_SPEED);
				transform.move_down(vy as f32 * DIAGONAL_SPEED);
			}

			// Attack.
			if input.action_is_down(&Actions::Primary).unwrap() {
				info!("Stab!\n");
			}
		}
	}
}
