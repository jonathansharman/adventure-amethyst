use crate::{
	component::{
		Hero,
		Position,
		Direction,
		Terrain,
	},
	resource::Region,
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{
		Entities,
		Join,
		ReadStorage,
		System,
		SystemData,
		WriteExpect,
		WriteStorage
	},
	renderer::SpriteRender,
};

/// Does collision detection and response for characters.
#[derive(SystemDesc)]
pub struct Travel;

impl<'a> System<'a> for Travel {
	type SystemData = (
		Entities<'a>,
		WriteExpect<'a, Region>,
		ReadStorage<'a, Hero>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Terrain>,
		WriteStorage<'a, Transform>,
		WriteStorage<'a, SpriteRender>,
	);

	fn run(&mut self, (entities, mut region, heroes, mut positions, mut directions, mut terrains, mut transforms, mut sprites): Self::SystemData) {
		for (hero_entity, _hero) in (&*entities, &heroes).join() {
			region.take_exit(
				hero_entity,
				&entities,
				&mut positions,
				&mut directions,
				&mut terrains,
				&mut transforms,
				&mut sprites,
			);
		}
	}
}
