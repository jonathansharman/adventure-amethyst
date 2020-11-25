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
	ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
	renderer::SpriteRender,
};
use nalgebra::base::Vector3;

/// Moves the camera.
#[derive(SystemDesc)]
pub struct Animation;

impl<'a> System<'a> for Animation {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, crate::component::Animation>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Direction>,
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Transform>,
		Read<'a, Time>,
	);

	fn run(&mut self, (
		entities,
		mut sto_animation,
		sto_position,
		sto_direction,
		mut sto_sprite_render,
		mut sto_transform,
		time,
	): Self::SystemData) {
		for (id, animation, position, direction) in (
			&entities,
			&mut sto_animation,
			&sto_position,
			&sto_direction,
		).join() {
			// Update animation.
			animation.advance(time.delta_time());
			animation.set_direction(*direction);
			// Set transform according to position.
			let transform = sto_transform.get_mut(id).unwrap();
			transform.set_translation_xyz(position.x, position.y, 0.5);
			transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			// Set the sprite render.
			sto_sprite_render.insert(id, animation.current_sprite_render()).unwrap();
		}
	}
}
