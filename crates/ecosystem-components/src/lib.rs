//! Core ECS components and data structures for ecosystem simulation
//!
//! This crate provides the foundational building blocks:
//! - Physical components (Position, Health, Energy)
//! - Behavioral components (AI, Movement)
//! - Configuration system (sealed traits)
//! - Organism types and markers

pub mod components;
pub mod config;
pub mod organisms;
//pub mod physical;
pub mod utils;

// Re-export everything for convenient usage
pub mod prelude {
    pub use crate::components::*;
    pub use crate::config::*;
    pub use crate::organisms::*;
    pub use crate::utils::*;

    // Bevy re-exports
    pub use bevy_ecs::prelude::*;
    pub use bevy_math::*;
    pub use bevy_utils::{HashMap, HashSet};

    // External utilities
    pub use anyhow::{Context, Result};
    pub use rand::prelude::*;
}

// Version info for compatibility checking
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
