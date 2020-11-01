use crate::{
	component::{
		Animation,
		Collider,
		Direction,
		Frame,
		Hero,
		HeroState,
		KnockedBack,
		Position,
		SwordAttack,
		Velocity,
	},
	constants::*,
	input_bindings::{InputBindings, Actions},
	resource::SpriteSheets,
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
	input::InputHandler,
	renderer::SpriteRender,
	shred::{Read},
};

use std::convert::TryFrom;
use std::time::Duration;

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct HeroControl;

impl<'a> System<'a> for HeroControl {
	type SystemData = (
		Read<'a, InputHandler<InputBindings>>,
		Entities<'a>,
		ReadExpect<'a, SpriteSheets>,
		WriteStorage<'a, Hero>,
		ReadStorage<'a, KnockedBack>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Velocity>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, SwordAttack>,
		WriteStorage<'a, Collider>,
		WriteStorage<'a, Animation>,
		WriteStorage<'a, Transform>,
		WriteStorage<'a, SpriteRender>,
	);

	fn run(&mut self, (
		input,
		entities,
		sprite_sheets,
		mut sto_hero,
		sto_knock_back,
		mut sto_position,
		mut sto_velocity,
		mut sto_direction,
		mut sto_sword_attack,
		mut sto_collider,
		mut sto_animation,
		mut sto_transform,
		mut sto_sprite,
	): Self::SystemData) {
		// Tuning parameters
		const ORTHOGONAL_SPEED: f32 = 5.0;
		const DIAGONAL_SPEED: f32 = 0.70710678118 * ORTHOGONAL_SPEED;
		const THRUST_FRAMES: u32 = 9;
		const THRUST_SPEED: f32 = 2.0 * ORTHOGONAL_SPEED;

		let components_iter = (
			&entities,
			&mut sto_hero,
			// No control while being knocked back
			!&sto_knock_back,
			&mut sto_velocity,
		).join();
		for (hero_id, hero, _knock_back, velocity) in components_iter {
			match hero.state {
				// Free for player to control
				HeroState::FreelyMoving => {
					// Sword thrust
					if input.action_is_down(&Actions::Primary).unwrap() {
						let sword_sprite = SpriteRender {
							sprite_sheet: sprite_sheets.sword.clone(),
							sprite_number: 0,
						};

						let hero_direction = sto_direction.get(hero_id).unwrap();
						let sword_collider = match hero_direction {
							Direction::Up | Direction::Down => Collider {
								half_width: SWORD_HALF_WIDTH,
								half_height: SWORD_HALF_LENGTH,
							},
							Direction::Left | Direction::Right => Collider {
								half_width: SWORD_HALF_LENGTH,
								half_height: SWORD_HALF_WIDTH,
							},
						};
						let sword_animation = Animation::new(vec!(
							Frame {
								up: 0,
								down: 1,
								left: 2,
								right: 3,
								duration: Duration::from_secs(1),
							}
						));
						let sword_attack_id = entities
							.build_entity()
							.with(SwordAttack::new(hero_id), &mut sto_sword_attack)
							.with(Position { x: 0.0, y: 0.0 }, &mut sto_position)
							.with(*hero_direction, &mut sto_direction)
							.with(sword_collider, &mut sto_collider)
							.with(sword_animation, &mut sto_animation)
							.with(Transform::default(), &mut sto_transform)
							.with(sword_sprite, &mut sto_sprite)
							.build();
						hero.state = HeroState::Thrusting {
							sword_attack_id,
							frames_left: THRUST_FRAMES,
						};
						// Skip movement checks if thrusting.
						continue;
					}
					// Movement
					let hero_direction = sto_direction.get_mut(hero_id).unwrap();
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
					match (*hero_direction, vx, vy) {
						// Moving in a cardinal direction.
						(_, 0, -1) => *hero_direction = Direction::Down,
						(_, 0, 1) => *hero_direction = Direction::Up,
						(_, -1, 0) => *hero_direction = Direction::Left,
						(_, 1, 0) => *hero_direction = Direction::Right,
						// Facing up but moving down-left, down, or right.
						(Direction::Up, _, -1) => *hero_direction = Direction::Down,
						// Facing down but moving up-left, up, or right.
						(Direction::Down, _, 1) => *hero_direction = Direction::Up,
						// Facing right but moving up-left, left, or down-left.
						(Direction::Right, -1, _) => *hero_direction = Direction::Left,
						// Facing left but moving up-right, right, or down-right.
						(Direction::Left, 1, _) => *hero_direction = Direction::Right,
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
				},
				// In the middle of a thrust
				HeroState::Thrusting { sword_attack_id, ref mut frames_left } => {
					let hero_direction = sto_direction.get(hero_id).unwrap();
					// Rush forward.
					*velocity = match hero_direction {
						Direction::Up => Velocity { x: 0.0, y: THRUST_SPEED },
						Direction::Down => Velocity { x: 0.0, y: -THRUST_SPEED },
						Direction::Left => Velocity { x: -THRUST_SPEED, y: 0.0 },
						Direction::Right => Velocity { x: THRUST_SPEED, y: 0.0 },
					};
					// Reduce frames left and return control if finished thrusting.
					*frames_left -= 1;
					if *frames_left == 0 {
						hero.state = HeroState::FreelyMoving;
						entities.delete(sword_attack_id).unwrap();
					}
				},
			}
		}
	}
}
