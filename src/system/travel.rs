use crate::{
	component::{
		Animation,
		behavior::Wander,
		Character,
		collider::RectangleCollider,
		Direction,
		Enemy,
		Health,
		Heart,
		Hero,
		Position,
		SlashAttack,
		Terrain,
		ThrustAttack,
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
		WriteStorage<'a, Animation>,
		WriteStorage<'a, Character>,
		WriteStorage<'a, Direction>,
		WriteStorage<'a, Enemy>,
		WriteStorage<'a, Health>,
		WriteStorage<'a, Heart>,
		WriteStorage<'a, Hero>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, RectangleCollider>,
		ReadStorage<'a, SlashAttack>,
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Terrain>,
		ReadStorage<'a, ThrustAttack>,
		WriteStorage<'a, Transform>,
		WriteStorage<'a, Velocity>,
		WriteStorage<'a, Wander>,
	);

	fn run(&mut self, (
		entities,
		mut region,
		sprite_sheets,
		mut sto_animation,
		mut sto_character,
		mut sto_direction,
		mut sto_enemy,
		mut sto_health,
		mut sto_heart,
		mut sto_hero,
		mut sto_position,
		mut sto_rectangle_collider,
		mut sto_slash_attack,
		mut sto_sprite_render,
		mut sto_terrain,
		mut sto_thrust_attack,
		mut sto_transform,
		mut sto_velocity,
		mut sto_wander,
	): Self::SystemData) {
		for (hero_id, hero) in (&*entities, &mut sto_hero).join() {
			region.take_exit(
				hero_id,
				hero,
				&entities,
				&sprite_sheets,
				&mut sto_animation,
				&mut sto_character,
				&mut sto_direction,
				&mut sto_enemy,
				&mut sto_health,
				&mut sto_heart,
				&mut sto_position,
				&mut sto_rectangle_collider,
				&mut sto_slash_attack,
				&mut sto_sprite_render,
				&mut sto_terrain,
				&mut sto_thrust_attack,
				&mut sto_transform,
				&mut sto_velocity,
				&mut sto_wander,
			);
		}
	}
}
