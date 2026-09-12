//! UI-related utilities

use gpui::App;

use crate::theme::ActiveTheme;

/// Returns true if the current theme is light.
pub fn is_light(cx: &mut App) -> bool {
    cx.theme().appearance.is_light()
}
