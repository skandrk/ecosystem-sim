//! Simple activity tracking - current state only

use bevy_ecs::prelude::*;
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

/// Current activity state (minimal tracking for MVP)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct CurrentActivity {
    pub activity: ActivityType,
}

impl CurrentActivity {
    pub fn new(activity: ActivityType) -> Self {
        Self { activity }
    }

    pub fn set_activity(&mut self, activity: ActivityType) {
        self.activity = activity;
    }
}

/// Basic activity types (minimal set for MVP)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum ActivityType {
    /// Entity is currently moving
    Moving,

    /// Entity is resting to recover energy
    Resting,
}

impl Default for ActivityType {
    fn default() -> Self {
        ActivityType::Resting // Default to resting (matches ActionType::Rest default)
    }
}

impl Default for CurrentActivity {
    fn default() -> Self {
        Self::new(ActivityType::default())
    }
}
