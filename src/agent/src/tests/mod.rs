//! Test suite for the Rae agent
//!
//! This module provides comprehensive testing capabilities including
//! unit tests, integration tests, and security tests.

pub mod unit;
pub mod integration;
pub mod security;

// Re-export test utilities
pub use unit::UnitTestSuite;
pub use integration::IntegrationTestSuite;
pub use security::SecurityTestSuite; 