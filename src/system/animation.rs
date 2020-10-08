use crate::{
	component::{Direction, Enemy, Hero, Position},
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
	renderer::SpriteRender,
};
use nalgebra::base::Vector3;

/// Moves the camera.
#[derive(SystemDesc)]
pub struct Animation;

impl<'a> System<'a> for Animation {
	type SystemData = (
		ReadStorage<'a, Hero>,
		ReadStorage<'a, Enemy>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Direction>,
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (
		all_heroes,
		all_enemies,
		all_positions,
		all_directions,
		mut all_sprites,
		mut all_transforms,
	): Self::SystemData) {
		// Hero animation.
		let components_iter = (
			&all_heroes,
			&all_positions,
			&all_directions,
			&mut all_sprites,
			&mut all_transforms,
		).join();
		for (_hero, position, direction, sprite, transform) in components_iter {
			// Update translation according to position.
			transform.set_translation_xyz(position.x, position.y, 0.5);
			// Increase scale.
			transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			// Update sprite according to direction.
			sprite.sprite_number = *direction as usize;
		}
		// Enemy animation.
		let components_iter = (
			&all_enemies,
			&all_positions,
			&all_directions,
			&mut all_sprites,
			&mut all_transforms,
		).join();
		for (_enemy, position, direction, sprite, transform) in components_iter {
			// Update translation according to position.
			transform.set_translation_xyz(position.x, position.y, 0.5);
			// Increase scale.
			transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
			// Update sprite according to direction.
			sprite.sprite_number = *direction as usize;
		}
	}
}
