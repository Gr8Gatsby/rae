//! Rae Agent Library
//!
//! Core library for the Rae agent, providing the foundation for
//! local-first, privacy-respecting AI assistance.

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
    #[derive(Debug)]
    pub enum RaeError {
        Config(String),
        Module(String),
        Storage(String),
        Schema(String),
        Security(String),
        Protocol(String),
        Io(std::io::Error),
        Serialization(serde_json::Error),
    }

    impl std::fmt::Display for RaeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                RaeError::Config(msg) => write!(f, "Configuration error: {}", msg),
                RaeError::Module(msg) => write!(f, "Module error: {}", msg),
                RaeError::Storage(msg) => write!(f, "Storage error: {}", msg),
                RaeError::Schema(msg) => write!(f, "Schema validation error: {}", msg),
                RaeError::Security(msg) => write!(f, "Security error: {}", msg),
                RaeError::Protocol(msg) => write!(f, "Protocol error: {}", msg),
                RaeError::Io(err) => write!(f, "IO error: {}", err),
                RaeError::Serialization(err) => write!(f, "Serialization error: {}", err),
            }
        }
    }

    impl std::error::Error for RaeError {}

    impl From<std::io::Error> for RaeError {
        fn from(err: std::io::Error) -> Self {
            RaeError::Io(err)
        }
    }

    impl From<serde_json::Error> for RaeError {
        fn from(err: serde_json::Error) -> Self {
            RaeError::Serialization(err)
        }
    }

    pub type Result<T> = std::result::Result<T, RaeError>;
} 