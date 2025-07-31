//! Core functionality for the Rae agent
//!
//! This module contains the essential components that make up the Rae agent:
//! - Agent: Main agent logic and coordination
//! - Scheduler: Task scheduling and automation
//! - Storage: Local data storage and management
//! - Messaging: Inter-module communication

pub mod agent;
pub mod scheduler;
pub mod storage;
pub mod messaging;

// Re-export main types
pub use agent::Agent;
pub use scheduler::Scheduler;
pub use storage::Storage;
pub use messaging::MessageBus; 