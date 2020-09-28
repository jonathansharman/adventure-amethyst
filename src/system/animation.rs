use crate::{
	component::{Direction, Hero, Position},
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
	renderer::SpriteRender,
};

/// Moves the camera.
#[derive(SystemDesc)]
pub struct Animation;

impl<'a> System<'a> for Animation {
	type SystemData = (
		ReadStorage<'a, Hero>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Direction>,
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (heroes, positions, directions, mut sprites, mut transforms): Self::SystemData) {
		for (_, position, direction, sprite, transform) in (&heroes, &positions, &directions, &mut sprites, &mut transforms).join() {
			// Update translation according to position.
			transform.set_translation_xyz(position.x, position.y, 0.5);
			// Update sprite according to direction.
			sprite.sprite_number = *direction as usize;
		}
	}
}
