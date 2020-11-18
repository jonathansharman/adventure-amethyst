use crate::component::{Direction, TileCoords};

use serde::Deserialize;

/// An entrance to a region.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct Entrance {
	pub location: TileCoords,
	pub direction: Direction,
}
