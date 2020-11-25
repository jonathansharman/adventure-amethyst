use amethyst::ecs::Entity;

/// Resource for tracking the entity ID of the game's camera.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Camera {
	pub id: Entity,
}
