use crate::{
	component::{
		Terrain,
		TileCoords,
	},
	resource::{Entrance, Exit},
};

use serde::Deserialize;

/// Used for reading region data from a file.
#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
pub struct RegionData {
	pub col_count: usize,
	pub terrain: Vec<Terrain>,
	pub entrances: Vec<Entrance>,
	pub exits: Vec<Exit>,
	pub enemies: Vec<EnemyData>,
	pub heart_locations: Vec<TileCoords>,
}

/// Used for reading enemy data from a file.
#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
pub struct EnemyData {
	pub location: TileCoords,
}
