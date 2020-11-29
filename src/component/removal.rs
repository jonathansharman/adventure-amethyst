/// Type parameters for the `Removal` type.

use amethyst::ecs::Entity;

/// Used as a `Removal` type parameter to mark an entity to be removed when another entity is deleted.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TiedToEntity(pub Entity);

/// Used as a `Removal` type parameter to mark an entity to be removed when the current region is unloaded.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TiedToRegion;
