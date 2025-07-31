//! Schema validation and management for the Rae agent
//!
//! This module provides schema validation, management, and evolution
//! capabilities to ensure data consistency across all modules.

pub mod validator;
pub mod manager;
pub mod evolution;

// Re-export main types
pub use validator::SchemaValidator;
pub use manager::SchemaManager;
pub use evolution::SchemaEvolution; 