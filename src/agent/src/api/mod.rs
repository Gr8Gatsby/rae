//! API layer for the Rae agent
//!
//! This module provides REST and WebSocket APIs for local communication,
//! as well as protocol support for A2A and MCP.

pub mod rest;
pub mod websocket;
pub mod protocols;

// Re-export main types
pub use rest::RestApi;
pub use websocket::WebSocketApi;
pub use protocols::ProtocolBridge; 