//! Behavioral components - minimal implementation for MVP
//!
//! This module provides the basic building blocks for AI behavior:
//! - AI input data structures (what AI can perceive)
//! - AI output command structures (what AI can do)
//! - Basic capability constraints (what AI is capable of)
//! - Simple activity tracking (current state)

pub mod ai_inputs;
pub mod ai_outputs;
pub mod constraints;
pub mod tracking;

// Re-export all behavioral components
pub use ai_inputs::*;
pub use ai_outputs::*;
pub use constraints::*;
pub use tracking::*;
