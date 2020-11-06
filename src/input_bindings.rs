extern crate amethyst;
extern crate serde;
use std::fmt::{self, Display};

use amethyst::input::BindingTypes;
use serde::{Serialize, Deserialize};

/// The axis binding is empty because actions are used for movement.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Axes;

/// The set of input actions the player can perform.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Actions {
	Up,
	Down,
	Left,
	Right,
	Primary,
	Secondary,
	Strafe,
}

impl Display for Axes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "")
	}
}

impl Display for Actions {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

/// Player input bindings.
#[derive(Debug)]
pub struct InputBindings;

impl BindingTypes for InputBindings {
	type Axis = Axes;
	type Action = Actions;
}
