//! Marker components for entity identification and categorization

use bevy_ecs::prelude::*;
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

/// Base marker component for all simulation entities
///
/// This component identifies entities that participate in the ecosystem simulation.
/// Used for filtering queries to only include simulation entities.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Organism;

/// Marker for entities that can be consumed as food
///
/// Entities with this marker can be eaten by other organisms.
/// Useful for plants, dead organisms, or food sources.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Edible;

/// Marker for entities that are currently alive and active
///
/// Used to distinguish active organisms from corpses or inactive entities.
/// Helpful for performance - systems can query only living entities.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Alive;

/// Marker for newborn entities (just spawned)
///
/// Can be used for special handling of newly created organisms,
/// temporary immunity, or initialization logic.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Newborn;
