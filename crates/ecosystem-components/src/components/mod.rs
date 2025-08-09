//! ECS component definitions for the ecosystem simulation

pub mod behavioral;
pub mod markers;
pub mod physical;

// Re-export all components for easy use
pub use behavioral::*;
pub use markers::*;
pub use physical::*;
