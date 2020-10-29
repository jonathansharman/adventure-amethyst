use crate::{
	component::{
		Animation,
		behavior::Wander,
		Collider,
		Direction,
		Enemy,
		Hero,
		Position,
		Terrain,
		Velocity,
	},
	resource::{Region, SpriteSheets},
};

use amethyst::{
	core::Transform,
	derive::SystemDesc,
	ecs::{
		Entities,
		Join,
		ReadExpect,
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
		ReadExpect<'a, SpriteSheets>,
		ReadStorage<'a, Hero>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Velocity>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Collider>,
		WriteStorage<'a, Terrain>,
		WriteStorage<'a, Enemy>,
		WriteStorage<'a, Animation>,
		WriteStorage<'a, Wander>,
		WriteStorage<'a, Transform>,
		WriteStorage<'a, SpriteRender>,
	);

	fn run(&mut self, (
		entities,
		mut region,
		sprite_sheets,
		all_heroes,
		mut all_positions,
		mut all_velocities,
		mut all_directions,
		mut all_colliders,
		mut all_terrain,
		mut all_enemies,
		mut all_animations,
		mut all_wanders,
		mut all_transforms,
		mut all_sprites,
	): Self::SystemData) {
		for (hero_id, _hero) in (&*entities, &all_heroes).join() {
			region.take_exit(
				hero_id,
				&entities,
				&sprite_sheets,
				&mut all_terrain,
				&mut all_enemies,
				&mut all_wanders,
				&mut all_positions,
				&mut all_velocities,
				&mut all_directions,
				&mut all_colliders,
				&mut all_animations,
				&mut all_transforms,
				&mut all_sprites,
			);
		}
	}
}
