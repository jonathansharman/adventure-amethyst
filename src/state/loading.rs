use amethyst::{
	core::transform::Transform,
	ecs::prelude::Entity,
	prelude::*,
	renderer::Camera,
	window::ScreenDimensions,
};

use crate::{
	resource,
	state::Playing,
};

/// Loads some data/resources and switches to `Playing`.
#[derive(Default)]
pub struct Loading;

impl SimpleState for Loading {
	fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();

		let camera_entity = init_camera(data.world, &dimensions);
		data.world.insert(resource::Camera { entity: Some(camera_entity) });
		Trans::Switch(Box::new(Playing))
	}
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) -> Entity {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(dimensions.width(), dimensions.height()))
		.with(transform)
		.build()
}
