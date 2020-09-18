use amethyst::{
	core::transform::Transform,
	ecs::prelude::Entity,
	prelude::*,
	renderer::Camera,
	window::ScreenDimensions,
};

use super::*;

/// Loads some data/resources and switches to `Playing`.
#[derive(Default)]
pub struct Loading;

impl SimpleState for Loading {
	fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();

		let camera = init_camera(data.world, &dimensions);
		Trans::Switch(Box::new(Playing { camera }))
	}
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) -> Entity {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.5 * dimensions.width(), 0.5 * dimensions.height(), 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(dimensions.width(), dimensions.height()))
		.with(transform)
		.build()
}
