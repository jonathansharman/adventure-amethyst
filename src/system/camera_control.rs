use crate::{
	component::Hero,
	resource::Camera,
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

/// Moves the camera.
#[derive(SystemDesc)]
pub struct CameraControl;

impl<'a> System<'a> for CameraControl {
	type SystemData = (
		Read<'a, Camera>,
		ReadStorage<'a, Hero>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (camera, heroes, mut transforms): Self::SystemData) {
		// The camera follows the hero.
		if let Some(camera_entity) = camera.entity {
			if let Some((_, hero_transform)) = (&heroes, &transforms).join().next() {
				let hero_translation = hero_transform.translation().clone();
				let camera_transform = transforms.get_mut(camera_entity).unwrap();
				camera_transform.set_translation_x(hero_translation.x);
				camera_transform.set_translation_y(hero_translation.y);
			}
		}
	}
}
