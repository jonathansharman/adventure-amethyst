use crate::{
	component::{Position, Velocity},
	constants::*,
};

use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct KnockedBack {
	velocity: Velocity,
	frames_left: u32,
}

impl Component for KnockedBack {
	type Storage = DenseVecStorage<Self>;
}

impl KnockedBack {
	/// Creates a knockback in the direction from `from` to `to`, at standard speed (`constants::KNOCKBACK_SPEED`).
	pub fn from_positions(from: &Position, to: &Position) -> Self {
		let mut velocity = Velocity {
			x: to.x - from.x,
			y: to.y - from.y,
		};
		// Normalize knockback velocity to the desired speed.
		if velocity.x != 0.0 || velocity.y != 0.0 {
			let magnitude = f32::sqrt(velocity.x.powi(2) + velocity.y.powi(2));
			velocity.x *= KNOCKBACK_SPEED / magnitude;
			velocity.y *= KNOCKBACK_SPEED / magnitude;
		} else {
			// To and from are direction on top of each other. Knock towards the right arbitrarily.
			velocity.x = KNOCKBACK_SPEED;
		}
		Self {
			velocity,
			frames_left: KNOCKBACK_FRAMES,
		}
	}

	/// Updates velocity and time left and returns whether the knockback effect has finished. To be called once per frame.
	/// `velocity` - A reference to the velocity of the character that is being knocked back.
	#[must_use]
	pub fn update(&mut self, velocity: &mut Velocity) -> KnockedBackFinished {
		*velocity = self.velocity;
		self.frames_left -= 1;
		if self.frames_left == 0 {
			KnockedBackFinished::Yes
		} else {
			KnockedBackFinished::No
		}
	}
}

pub enum KnockedBackFinished {
	/// The knockback has finished.
	Yes,
	/// The knockback has not yet finished.
	No,
}
