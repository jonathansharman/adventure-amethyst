use crate::component::{
	animation::{DirectionalAnimation, SimpleAnimation},
	Direction,
	Position,
};

use amethyst::{
	core::{
		Transform,
	},
	derive::SystemDesc,
	ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
	renderer::SpriteRender,
};
use nalgebra::base::Vector3;

/// Moves the camera.
#[derive(SystemDesc)]
pub struct Animation;

impl<'a> System<'a> for Animation {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, DirectionalAnimation>,
		WriteStorage<'a, SimpleAnimation>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Direction>,
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (
		entities,
		mut sto_directional_animation,
		mut sto_simple_animation,
		sto_position,
		sto_direction,
		mut sto_sprite_render,
		mut sto_transform,
	): Self::SystemData) {
		// Update directional animations.
		for (id, animation, position, direction) in (
			&entities,
			&mut sto_directional_animation,
			&sto_position,
			&sto_direction,
		).join() {
			// Update animation.
			animation.advance();
			animation.set_direction(*direction);
			// Set transform according to position.
			let mut transform = Transform::default();
			transform.set_translation_xyz(position.x, position.y, 0.5);
			transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			sto_transform.insert(id, transform).unwrap();
			// Set the sprite render.
			sto_sprite_render.insert(id, animation.current_sprite_render()).unwrap();
		}
		// Update simple animations.
		for (id, animation, position) in (
			&entities,
			&mut sto_simple_animation,
			&sto_position,
		).join() {
			// Update animation.
			animation.advance();
			// Set transform according to position.
			let mut transform = Transform::default();
			transform.set_translation_xyz(position.x, position.y, 0.5);
			transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			sto_transform.insert(id, transform).unwrap();
			// Set the sprite render.
			sto_sprite_render.insert(id, animation.current_sprite_render()).unwrap();
		}
	}
}
