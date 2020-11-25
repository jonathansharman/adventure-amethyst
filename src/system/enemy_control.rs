use crate::{
	component::{
		Animation,
		ArrowAttack,
		behavior::{ShootArrows, ShouldShootArrow, Wander},
		collider::RectangleCollider,
		Direction,
		Faction,
		Frame,
		KnockedBack,
		Position,
		Velocity,
	},
	constants::*,
	resource::SpriteSheets,
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Entity, Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
	utils::removal::Removal,
};
use rand::Rng;

use std::time::Duration;

/// Controls the hero character based on player input.
#[derive(SystemDesc)]
pub struct EnemyControl;

impl<'a> System<'a> for EnemyControl {
	type SystemData = (
		Entities<'a>,
		ReadExpect<'a, SpriteSheets>,
		WriteStorage<'a, Removal<()>>,
		WriteStorage<'a, Removal<Entity>>,
		WriteStorage<'a, Wander>,
		WriteStorage<'a, ShootArrows>,
		ReadStorage<'a, KnockedBack>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Velocity>,
		WriteStorage<'a, RectangleCollider>,
		WriteStorage<'a, ArrowAttack>,
		WriteStorage<'a, Animation>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (
		entities,
		sprite_sheets,
		mut sto_unit_removal,
		mut sto_entity_removal,
		mut sto_wander,
		mut sto_shoot_arrows,
		sto_knocked_back,
		mut sto_direction,
		mut sto_position,
		mut sto_velocity,
		mut sto_rectangle_collider,
		mut sto_arrow_attack,
		mut sto_animation,
		mut sto_transform,
	): Self::SystemData) {
		const SPEED: f32 = 3.0;
		const TURN_THRESHOLD: f32 = 0.01;
		const STOP_THRESHOLD: f32 = TURN_THRESHOLD + 0.01;
		// Wander.
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
		// Shoot arrows.
		for (source_id, shoot_arrows, _no_knocked_back) in (
			&entities,
			&mut sto_shoot_arrows,
			// No control while being knocked back
			!&sto_knocked_back,
		).join() {
			if let ShouldShootArrow::Yes = shoot_arrows.update() {
				let position = *sto_position.get(source_id).unwrap();
				let direction = *sto_direction.get(source_id).unwrap();
				let velocity = match direction {
					Direction::Up => Velocity { x: 0.0, y: ARROW_SPEED },
					Direction::Down => Velocity { x: 0.0, y: -ARROW_SPEED },
					Direction::Left => Velocity { x: -ARROW_SPEED, y: 0.0 },
					Direction::Right => Velocity { x: ARROW_SPEED, y: 0.0 },
				};
				let collider = ArrowAttack::compute_collider(&direction);
				entities
					.build_entity()
					.with(Removal::new(()), &mut sto_unit_removal)
					.with(Removal::new(source_id), &mut sto_entity_removal)
					.with(ArrowAttack::new(Faction::Enemy), &mut sto_arrow_attack)
					.with(position, &mut sto_position)
					.with(velocity, &mut sto_velocity)
					.with(direction, &mut sto_direction)
					.with(collider, &mut sto_rectangle_collider)
					.with(Animation::new(sprite_sheets.arrow_attack.clone(), vec!(
						Frame {
							up: 0,
							down: 1,
							left: 2,
							right: 3,
							duration: Duration::from_secs(1),
						},
					)), &mut sto_animation)
					.with(Transform::default(), &mut sto_transform)
					.build();
			}
		}
	}
}
