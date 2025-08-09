//! Basic capability constraints - what the entity is capable of

use bevy_ecs::prelude::*;
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

/// Vision capability - how far an entity can see
#[derive(Component, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Vision {
    pub range: f32,
}

impl Vision {
    pub fn new(range: f32) -> Self {
        Self {
            range: range.max(0.0),
        }
    }

    pub fn set_range(&mut self, range: f32) {
        self.range = range.max(0.0);
    }
}

impl Default for Vision {
    fn default() -> Self {
        Self::new(100.0) // Default vision range
    }
}
