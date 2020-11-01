use crate::component::{
	Direction,
	Position,
};

use amethyst::{
	core::{
		timing::Time,
		Transform,
	},
	derive::SystemDesc,
	ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
	renderer::SpriteRender,
};
use nalgebra::base::Vector3;

/// Moves the camera.
#[derive(SystemDesc)]
pub struct Animation;

impl<'a> System<'a> for Animation {
	type SystemData = (
		WriteStorage<'a, crate::component::Animation>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Direction>,
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Transform>,
		Read<'a, Time>,
	);

	fn run(&mut self, (
		mut sto_animation,
		sto_position,
		sto_direction,
		mut sto_sprite,
		mut sto_transform,
		time,
	): Self::SystemData) {
		let components_iter = (
			&mut sto_animation,
			&sto_position,
			&sto_direction,
			&mut sto_sprite,
			&mut sto_transform,
		).join();
		for (animation, position, direction, sprite, transform) in components_iter {
			// Update animation.
			animation.advance(time.delta_time());
			animation.set_direction(*direction);
			// Update translation according to position.
			transform.set_translation_xyz(position.x, position.y, 0.5);
			// Increase scale.
			transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			// Update sprite according to direction.
			sprite.sprite_number = animation.current_sprite_number();
		}
	}
}
