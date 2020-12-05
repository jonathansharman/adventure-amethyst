use crate::{
	component::{
		animation::{DirectionalAnimation, DirectionalFrame},
		collider::{HalfDiskCollider, RectangleCollider},
		Direction,
		Hero,
		HeroState,
		KnockedBack,
		Position,
		removal::{TiedToEntity, TiedToRegion},
		SlashAttack,
		ThrustAttack,
		Velocity,
	},
	constants::*,
	input_bindings::{InputBindings, Actions},
	resource::SpriteSheets,
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
	input::InputHandler,
	shred::Read,
	utils::removal::Removal,
};

use std::convert::TryFrom;

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct HeroControl {
	primary_action_down_last_frame: bool,
}

impl HeroControl {
	pub fn new() -> Self {
		Self {
			primary_action_down_last_frame: false,
		}
	}
}

impl<'a> System<'a> for HeroControl {
	type SystemData = (
		Read<'a, InputHandler<InputBindings>>,
		Entities<'a>,
		ReadExpect<'a, SpriteSheets>,
		WriteStorage<'a, Removal<TiedToRegion>>,
		WriteStorage<'a, Removal<TiedToEntity>>,
		WriteStorage<'a, Hero>,
		ReadStorage<'a, KnockedBack>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Velocity>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, SlashAttack>,
		WriteStorage<'a, ThrustAttack>,
		WriteStorage<'a, RectangleCollider>,
		WriteStorage<'a, HalfDiskCollider>,
		WriteStorage<'a, DirectionalAnimation>,
	);

	fn run(&mut self, (
		input,
		entities,
		sprite_sheets,
		mut sto_removal_tied_to_region,
		mut sto_removal_tied_to_entity,
		mut sto_hero,
		sto_knock_back,
		mut sto_position,
		mut sto_velocity,
		mut sto_direction,
		mut sto_slash_attack,
		mut sto_thrust_attack,
		mut sto_rectangle_collider,
		mut sto_disk_arc_collider,
		mut sto_directional_animation,
	): Self::SystemData) {
		// Tuning parameters
		const ORTHOGONAL_SPEED: f32 = 5.0;
		const DIAGONAL_SPEED: f32 = 0.70710678118 * ORTHOGONAL_SPEED;
		const SLASH_FRAMES: u32 = 9;
		const THRUST_FRAMES: u32 = 9;
		const THRUST_SPEED: f32 = 2.0 * ORTHOGONAL_SPEED;

		let primary_action_down = input.action_is_down(&Actions::Primary).unwrap_or(false);

		for (hero_id, hero, _no_knocked_back, velocity) in (
			&entities,
			&mut sto_hero,
			// No control while being knocked back
			!&sto_knock_back,
			&mut sto_velocity,
		).join() {
			match hero.state {
				// Free for player to control
				HeroState::FreelyMoving => {
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
					if !input.action_is_down(&Actions::Strafe).unwrap() {
						// Not strafing.
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
					}
					// Update velocity.
					if vx == 0 || vy == 0 {
						velocity.x = f32::try_from(vx).unwrap() * ORTHOGONAL_SPEED;
						velocity.y = f32::try_from(vy).unwrap() * ORTHOGONAL_SPEED;
					} else {
						velocity.x = f32::try_from(vx).unwrap() * DIAGONAL_SPEED;
						velocity.y = f32::try_from(vy).unwrap() * DIAGONAL_SPEED;
					}
					// Check for sword attack.
					if !self.primary_action_down_last_frame && primary_action_down {
						let hero_direction = *sto_direction.get(hero_id).unwrap();
						let hero_position = *sto_position.get(hero_id).unwrap();
						let advancing = match hero_direction {
							Direction::Up => vy == 1,
							Direction::Down => vy == -1,
							Direction::Left => vx == -1,
							Direction::Right => vx == 1,
						};
						if advancing {
							// Hero is advancing -> thrust attack.
							let thrust_attack_animation = DirectionalAnimation::new(
								sprite_sheets.thrust_attack.clone(),
								vec!(DirectionalFrame {
									up: 0,
									down: 1,
									left: 2,
									right: 3,
									duration: None,
								}),
							);
							let thrust_attack_id = entities
								.build_entity()
								.with(ThrustAttack::new(hero_id), &mut sto_thrust_attack)
								.with(Removal::new(TiedToRegion), &mut sto_removal_tied_to_region)
								.with(Removal::new(TiedToEntity(hero_id)), &mut sto_removal_tied_to_entity)
								.with(hero_position, &mut sto_position)
								.with(hero_direction, &mut sto_direction)
								.with(ThrustAttack::compute_collider(&hero_direction), &mut sto_rectangle_collider)
								.with(thrust_attack_animation, &mut sto_directional_animation)
								.build();
							hero.state = HeroState::Thrusting {
								thrust_attack_id: thrust_attack_id,
								frames_left: THRUST_FRAMES,
							};
						} else {
							// Hero is strafing/retreating/standing still -> slash attack.
							let slash_attack_collider = HalfDiskCollider {
								radius: SLASH_ATTACK_RADIUS,
								direction: hero_direction,
							};
							let slash_attack_animation = DirectionalAnimation::new(
								sprite_sheets.slash_attack.clone(),
								vec!(DirectionalFrame {
									up: 0,
									down: 1,
									left: 2,
									right: 3,
									duration: None,
								}),
							);
							let slash_attack_id = entities
								.build_entity()
								.with(SlashAttack::new(hero_id), &mut sto_slash_attack)
								.with(Removal::new(TiedToRegion), &mut sto_removal_tied_to_region)
								.with(Removal::new(TiedToEntity(hero_id)), &mut sto_removal_tied_to_entity)
								.with(hero_position, &mut sto_position)
								.with(hero_direction, &mut sto_direction)
								.with(slash_attack_collider, &mut sto_disk_arc_collider)
								.with(slash_attack_animation, &mut sto_directional_animation)
								.build();
							hero.state = HeroState::Slashing {
								slash_attack_id,
								frames_left: SLASH_FRAMES,
							};
						}
					}
				},
				// In the middle of a slash
				HeroState::Slashing { slash_attack_id, ref mut frames_left } => {
					// Reduce frames left and return control if finished slashing.
					*frames_left -= 1;
					if *frames_left == 0 {
						hero.state = HeroState::FreelyMoving;
						entities.delete(slash_attack_id).unwrap();
					}
				},
				// In the middle of a thrust
				HeroState::Thrusting { thrust_attack_id, ref mut frames_left } => {
					let thrust_attack = sto_thrust_attack.get_mut(thrust_attack_id).unwrap();
					let hero_direction = sto_direction.get(hero_id).unwrap();
					// Rush forward if the thrust is active.
					*velocity = if thrust_attack.is_active() {
						match hero_direction {
							Direction::Up => Velocity { x: 0.0, y: THRUST_SPEED },
							Direction::Down => Velocity { x: 0.0, y: -THRUST_SPEED },
							Direction::Left => Velocity { x: -THRUST_SPEED, y: 0.0 },
							Direction::Right => Velocity { x: THRUST_SPEED, y: 0.0 },
						}
					} else {
						Velocity { x: 0.0, y: 0.0 }
					};
					// Reduce frames left and return control if finished thrusting.
					*frames_left -= 1;
					if *frames_left == 0 {
						hero.state = HeroState::FreelyMoving;
						entities.delete(thrust_attack_id).unwrap();
					}
				},
			}
		}

		self.primary_action_down_last_frame = primary_action_down;
	}
}
