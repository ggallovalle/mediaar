//! # UI – kbgpui primitives & components
//!
//! This module provides UI primitives and components used to build GPUI surfaces.
//! Layout and names follow Zed's `ui` crate.

mod components;
pub mod prelude;
mod styles;
mod traits;
pub mod utils;

pub use components::*;
pub use prelude::*;
pub use styles::*;

/// Bind UI key contexts (menus, etc.). Call after [`crate::theme::init`].
pub fn init(cx: &mut gpui::App) {
    components::init(cx);
}
