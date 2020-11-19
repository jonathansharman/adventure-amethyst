use crate::constants::*;

use amethyst::{
	ecs::{Component, DenseVecStorage, Entity, WriteStorage},
	renderer::{
		palette::Srgba,
		resources::Tint,
	},
};

/// Makes a character immune to damage for a number of frames.
pub struct Invulnerable {
	frames_left: u32,
}

impl Component for Invulnerable {
	type Storage = DenseVecStorage<Self>;
}

impl Invulnerable {
	pub fn new() -> Self {
		Self {
			frames_left: INVULNERABLE_FRAMES,
		}
	}

	/// Updates time left and returns whether the invulnerable effect has worn off.
	/// `id` - The ID of the entity with invulnerability.
	/// `sto_tint` - Tint component storage, used to update the blinking effect.
	#[must_use]
	pub fn update<'a>(&mut self, id: Entity, sto_tint: &mut WriteStorage<'a, Tint>) -> InvulnerableFinished {
		self.frames_left -= 1;
		if (self.frames_left / INVULNERABLE_BLINK_PERIOD) % 2 == 0 {
			sto_tint.remove(id);
		} else {
			let lum = INVULNERABLE_BLINK_LUMINANCE;
			sto_tint.insert(id, Tint(Srgba::new(lum, lum, lum, 1.0))).unwrap();
		}
		if self.frames_left == 0 {
			InvulnerableFinished::Yes
		} else {
			InvulnerableFinished::No
		}
	}
}

pub enum InvulnerableFinished {
	/// The invulnerability has worn off.
	Yes,
	/// The invulnerability has not yet worn off.
	No,
}
