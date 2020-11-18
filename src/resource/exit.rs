use crate::component::TileCoords;

use serde::Deserialize;

/// An exit from one region to another.
#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
pub struct Exit {
	pub location: TileCoords,
	pub target_region: String,
	pub target_entrance_idx: usize,
}
