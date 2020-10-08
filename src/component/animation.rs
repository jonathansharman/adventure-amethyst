use crate::component::Direction;

use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

use std::time::Duration;

impl Component for Animation {
	type Storage = DenseVecStorage<Self>;
}

#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
pub struct Animation {
	frames: Vec<Frame>,
	frame_number: usize,
	frame_progress: Duration,
	direction: Direction,
}

impl Animation {
	pub fn new(frames: Vec<Frame>) -> Animation {
		Animation {
			frames,
			frame_number: 0,
			frame_progress: Duration::from_secs(0),
			direction: Direction::Up,
		}
	}

	pub fn current_sprite_number(&self) -> usize {
		let frame = self.frames[self.frame_number];
		match self.direction {
			Direction::Up => frame.up,
			Direction::Down => frame.down,
			Direction::Left => frame.left,
			Direction::Right => frame.right,
		}
	}

	pub fn advance(&mut self, elapsed_time: Duration) {
		self.frame_progress += elapsed_time;
		while self.frame_progress >= self.frames[self.frame_number].duration {
			self.frame_progress -= self.frames[self.frame_number].duration;
			self.frame_number = (self.frame_number + 1) % self.frames.len();
		}
	}

	pub fn set_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct Frame {
	/// Sprite number for the up direction.
	pub up: usize,
	/// Sprite number for the down direction.
	pub down: usize,
	/// Sprite number for the left direction.
	pub left: usize,
	/// Sprite number for the right direction.
	pub right: usize,
	pub duration: Duration,
}
