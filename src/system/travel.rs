use crate::{
	component::{
		Animation,
		behavior::Wander,
		collider::RectangleCollider,
		Direction,
		Enemy,
		Health,
		Heart,
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
		WriteStorage<'a, RectangleCollider>,
		WriteStorage<'a, Terrain>,
		WriteStorage<'a, Enemy>,
		WriteStorage<'a, Health>,
		WriteStorage<'a, Heart>,
		WriteStorage<'a, Animation>,
		WriteStorage<'a, Wander>,
		WriteStorage<'a, Transform>,
		WriteStorage<'a, SpriteRender>,
	);

	fn run(&mut self, (
		entities,
		mut region,
		sprite_sheets,
		sto_hero,
		mut sto_position,
		mut sto_velocity,
		mut sto_direction,
		mut sto_rectangle_collider,
		mut sto_terrain,
		mut sto_enemy,
		mut sto_health,
		mut sto_heart,
		mut sto_animation,
		mut sto_wander,
		mut sto_transform,
		mut sto_sprite,
	): Self::SystemData) {
		for (hero_id, _hero) in (&*entities, &sto_hero).join() {
			region.take_exit(
				hero_id,
				&entities,
				&sprite_sheets,
				&mut sto_terrain,
				&mut sto_enemy,
				&mut sto_health,
				&mut sto_heart,
				&mut sto_wander,
				&mut sto_position,
				&mut sto_velocity,
				&mut sto_direction,
				&mut sto_rectangle_collider,
				&mut sto_animation,
				&mut sto_transform,
				&mut sto_sprite,
			);
		}
	}
}
