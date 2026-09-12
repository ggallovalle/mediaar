//! Elevation tokens (Zed `ElevationIndex`).

use gpui::{App, Hsla};

use crate::theme::ActiveTheme;

/// Physical closeness of a surface to the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevationIndex {
    Background,
    Surface,
    EditorSurface,
    ElevatedSurface,
    ModalSurface,
}

impl ElevationIndex {
    pub fn bg(self, cx: &App) -> Hsla {
        match self {
            Self::Background => cx.theme().colors().background,
            Self::Surface => cx.theme().colors().surface_background,
            Self::EditorSurface => cx.theme().colors().editor_background,
            Self::ElevatedSurface | Self::ModalSurface => {
                cx.theme().colors().elevated_surface_background
            }
        }
    }
}
