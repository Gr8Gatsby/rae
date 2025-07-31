//! User interface components for the Rae agent
//!
//! This module provides native Web Components for the user interface,
//! following the UI philosophy defined in the functional specification.

pub mod components;
pub mod layout;
pub mod themes;

// Re-export main types
pub use components::ComponentRegistry;
pub use layout::LayoutManager;
pub use themes::ThemeManager; 