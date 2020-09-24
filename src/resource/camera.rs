use amethyst::ecs::Entity;

#[derive(Default)]
pub struct Camera {
	pub entity: Option<Entity>,
}
