//! AI input data structures - what the AI can perceive

use bevy_math::Vec2;
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

use crate::organisms::OrganismType;

/// Complete observation data passed to AI systems
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct ObservationData {
    pub visible_entities: Vec<EntityObservation>,
    pub self_state: SelfState,
}

impl ObservationData {
    pub fn new(self_state: SelfState) -> Self {
        Self {
            visible_entities: Vec::new(),
            self_state,
        }
    }

    pub fn add_observation(&mut self, observation: EntityObservation) {
        self.visible_entities.push(observation);
    }

    pub fn clear_observations(&mut self) {
        self.visible_entities.clear();
    }
}

/// Single entity observation within vision range
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct EntityObservation {
    pub organism_type: OrganismType,
    pub relative_position: Vec2,
}

impl EntityObservation {
    pub fn new(organism_type: OrganismType, relative_position: Vec2) -> Self {
        Self {
            organism_type,
            relative_position,
        }
    }

    /// Calculate distance from relative position
    pub fn distance(&self) -> f32 {
        self.relative_position.length()
    }

    /// Get unit direction vector toward this entity
    pub fn direction(&self) -> Vec2 {
        self.relative_position.normalize_or_zero()
    }
}

/// Current state of the observing entity
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct SelfState {
    pub health_ratio: f32, // 0.0 to 1.0
    pub energy_ratio: f32, // 0.0 to 1.0
    pub current_position: Vec2,
}

impl SelfState {
    pub fn new(health_ratio: f32, energy_ratio: f32, current_position: Vec2) -> Self {
        Self {
            health_ratio: health_ratio.clamp(0.0, 1.0),
            energy_ratio: energy_ratio.clamp(0.0, 1.0),
            current_position,
        }
    }
}
