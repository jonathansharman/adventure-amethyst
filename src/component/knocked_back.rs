use crate::component::Velocity;

use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct KnockedBack {
	pub velocity: Velocity,
	pub frames_left: u32,
}

impl Component for KnockedBack {
	type Storage = DenseVecStorage<Self>;
}
