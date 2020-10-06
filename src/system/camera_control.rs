use crate::{
	component::{Hero, Position},
	resource::Camera,
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

/// Moves the camera.
#[derive(SystemDesc)]
pub struct CameraControl;

impl<'a> System<'a> for CameraControl {
	type SystemData = (
		ReadExpect<'a, Camera>,
		ReadStorage<'a, Hero>,
		ReadStorage<'a, Position>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (camera, all_heroes, all_positions, mut all_transforms): Self::SystemData) {
		// The camera follows the hero.
		if let Some((_hero, hero_position)) = (&all_heroes, &all_positions).join().next() {
			let camera_transform = all_transforms.get_mut(camera.entity).unwrap();
			camera_transform.set_translation_x(hero_position.x);
			camera_transform.set_translation_y(hero_position.y);
		}
	}
}
