use crate::component::Direction;

use amethyst::{
	assets::Handle,
	ecs::{Component, DenseVecStorage},
	renderer::{SpriteRender, SpriteSheet},
};
use serde::Deserialize;

/// An animation that can use different sprites for the four different directions.
#[derive(PartialEq, Clone, Debug)]
pub struct DirectionalAnimation {
	sprite_sheet: Handle<SpriteSheet>,
	frames: Vec<DirectionalFrame>,
	frame_number: usize,
	frame_progress: u32,
	direction: Direction,
}

impl Component for DirectionalAnimation {
	type Storage = DenseVecStorage<Self>;
}

impl DirectionalAnimation {
	pub fn new(sprite_sheet: Handle<SpriteSheet>, frames: Vec<DirectionalFrame>) -> Self {
		Self {
			sprite_sheet,
			frames,
			frame_number: 0,
			frame_progress: 0,
			direction: Direction::Up,
		}
	}

	pub fn current_sprite_render(&self) -> SpriteRender {
		let frame = self.frames[self.frame_number];
		let sprite_number = match self.direction {
			Direction::Up => frame.up,
			Direction::Down => frame.down,
			Direction::Left => frame.left,
			Direction::Right => frame.right,
		};
		SpriteRender {
			sprite_sheet: self.sprite_sheet.clone(),
			sprite_number,
		}
	}

	/// Advances the animation by one game frame.
	pub fn advance(&mut self) {
		if let Some(frame_duration) = self.frames[self.frame_number].duration {
			self.frame_progress += 1;
			if self.frame_progress == frame_duration {
				self.frame_progress = 0;
				self.frame_number = (self.frame_number + 1) % self.frames.len();
			}
		}
	}

	pub fn set_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct DirectionalFrame {
	/// Sprite number for the up direction.
	pub up: usize,
	/// Sprite number for the down direction.
	pub down: usize,
	/// Sprite number for the left direction.
	pub left: usize,
	/// Sprite number for the right direction.
	pub right: usize,
	/// Duration of this animation frame, in game frames. If `None`, the animation stops on this frame.
	pub duration: Option<u32>,
}
