use crate::constants::*;

use amethyst::ecs::{Component, DenseVecStorage};
use rand::distributions::{Distribution, Uniform};

/// Behavior that causes an enemy to shoot arrows at random intervals.
#[derive(Copy, Clone, Debug)]
pub struct ShootArrows {
	delay: u32,
	distribution: Uniform<u32>,
}

impl Component for ShootArrows {
	type Storage = DenseVecStorage<Self>;
}

impl ShootArrows {
	pub fn new() -> Self {
		let distribution = Uniform::from(MIN_ARROW_DELAY..MAX_ARROW_DELAY);
		let delay = distribution.sample(&mut rand::thread_rng());
		Self {
			delay,
			distribution,
		}
	}

	pub fn update(&mut self) -> ShouldShootArrow {
		if self.delay == 0 {
			self.delay = self.distribution.sample(&mut rand::thread_rng());
			ShouldShootArrow::Yes
		} else {
			self.delay -= 1;
			ShouldShootArrow::No
		}
	}
}

pub enum ShouldShootArrow {
	Yes,
	No,
}
