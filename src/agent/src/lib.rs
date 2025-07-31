//! Rae Agent Library
//!
//! Core library for the Rae agent, providing the foundation for
//! local-first, privacy-respecting AI assistance.

pub mod cli;
pub mod core;
pub mod api;
pub mod modules;
pub mod schemas;
pub mod ui;
pub mod tests;

// Re-export main types for convenience
pub use core::agent::Agent;
pub use core::scheduler::Scheduler;
pub use core::storage::Storage;
pub use modules::ModuleManager;

/// Current version of the Rae agent
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration values
pub mod config {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub data_dir: String,
        pub log_level: String,
        pub privacy_level: PrivacyLevel,
        pub max_modules: usize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum PrivacyLevel {
        Strict,    // No external communication
        Standard,  // Minimal external communication
        Open,      // Full external communication (user consent required)
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                data_dir: "~/.rae".to_string(),
                log_level: "info".to_string(),
                privacy_level: PrivacyLevel::Strict,
                max_modules: 10,
            }
        }
    }
}

/// Error types for the Rae agent
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum RaeError {
        #[error("Configuration error: {0}")]
        Config(String),
        #[error("Module error: {0}")]
        Module(String),
        #[error("Storage error: {0}")]
        Storage(String),
        #[error("Schema validation error: {0}")]
        Schema(String),
        #[error("Security error: {0}")]
        Security(String),
        #[error("Protocol error: {0}")]
        Protocol(String),
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        #[error("Serialization error: {0}")]
        Serialization(#[from] serde_json::Error),
    }

    pub type Result<T> = std::result::Result<T, RaeError>;
}

/// Common types used throughout the agent
pub mod types {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModuleInfo {
        pub name: String,
        pub version: String,
        pub description: String,
        pub permissions: Vec<String>,
        pub status: ModuleStatus,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ModuleStatus {
        Active,
        Inactive,
        Error(String),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ActivityData {
        pub timestamp: chrono::DateTime<chrono::Utc>,
        pub module: String,
        pub data: HashMap<String, serde_json::Value>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Digest {
        pub period: String,
        pub start_date: chrono::DateTime<chrono::Utc>,
        pub end_date: chrono::DateTime<chrono::Utc>,
        pub activities: Vec<ActivityData>,
        pub summary: String,
    }
} 