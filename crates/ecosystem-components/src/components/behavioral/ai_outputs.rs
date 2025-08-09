//! AI output command structures - what the AI decides to do

use bevy_ecs::prelude::*;
use bevy_math::Vec2;
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

/// Command issued by AI system to be executed by physics systems
#[derive(Component, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct ActionCommand {
    pub action_type: ActionType,
}

impl ActionCommand {
    pub fn new(action_type: ActionType) -> Self {
        Self { action_type }
    }

    pub fn move_to(direction: Vec2) -> Self {
        Self::new(ActionType::Move(direction))
    }

    pub fn rest() -> Self {
        Self::new(ActionType::Rest)
    }
}

/// Types of actions an entity can perform (minimal set for MVP)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum ActionType {
    /// Move in specified direction (vector will be constrained by movement capabilities)
    Move(Vec2),

    /// Rest to recover energy (no movement)
    Rest,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Rest
    }
}

impl Default for ActionCommand {
    fn default() -> Self {
        Self::new(ActionType::default())
    }
}
