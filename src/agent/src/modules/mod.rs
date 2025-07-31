//! Module system for the Rae agent
//!
//! This module provides the extensible module architecture that allows
//! Rae to be extended with new capabilities while maintaining security
//! and privacy through sandboxing.

pub mod manager;
pub mod runner;
pub mod sandbox;
pub mod builtin;

// Re-export main types
pub use manager::ModuleManager;
pub use runner::ModuleRunner;
pub use sandbox::ModuleSandbox;
pub use builtin::BuiltinModules; 