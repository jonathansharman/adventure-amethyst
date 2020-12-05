use amethyst::{
	assets::Handle,
	ecs::{Component, DenseVecStorage},
	renderer::{SpriteRender, SpriteSheet},
};
use serde::Deserialize;

/// A basic animation. See also `DirectionalAnimation`.
#[derive(PartialEq, Clone, Debug)]
pub struct SimpleAnimation {
	sprite_sheet: Handle<SpriteSheet>,
	frames: Vec<SimpleFrame>,
	frame_number: usize,
	frame_progress: u32,
}

impl Component for SimpleAnimation {
	type Storage = DenseVecStorage<Self>;
}

impl SimpleAnimation {
	pub fn new(sprite_sheet: Handle<SpriteSheet>, frames: Vec<SimpleFrame>) -> Self {
		Self {
			sprite_sheet,
			frames,
			frame_number: 0,
			frame_progress: 0,
		}
	}

	pub fn current_sprite_render(&self) -> SpriteRender {
		let frame = self.frames[self.frame_number];
		SpriteRender {
			sprite_sheet: self.sprite_sheet.clone(),
			sprite_number: frame.sprite_number,
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
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct SimpleFrame {
	pub sprite_number: usize,
	/// Duration of this animation frame, in game frames. If `None`, the animation stops on this frame.
	pub duration: Option<u32>,
}
