use gpui::{App, Hsla};

use crate::theme::ActiveTheme;

/// Sets a color that has a consistent meaning across all themes.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    /// The default text color. Might be known as "foreground" or "primary" in
    /// some theme systems.
    Default,
    /// A text color used for accents, such as links or highlights.
    Accent,
    /// A custom color specified by an HSLA value.
    Custom(Hsla),
    /// A color used for disabled UI elements or text.
    Disabled,
    /// A color used for hint or suggestion text.
    Hint,
    /// A color used for informational messages or status indicators.
    Info,
    /// A color used for text or UI elements that should be visually muted.
    Muted,
    /// A color used to indicate selected text or UI elements.
    Selected,
}

impl Color {
    /// Returns the Color's HSLA value.
    pub fn color(&self, cx: &App) -> Hsla {
        match self {
            Color::Default => cx.theme().colors().text,
            Color::Muted => cx.theme().colors().text_muted,
            Color::Disabled => cx.theme().colors().text_disabled,
            Color::Hint => cx.theme().status().info,
            Color::Info => cx.theme().status().info,
            Color::Accent => cx.theme().colors().text_accent,
            Color::Selected => cx.theme().colors().text_accent,
            Color::Custom(color) => *color,
        }
    }
}

impl From<Hsla> for Color {
    fn from(color: Hsla) -> Self {
        Color::Custom(color)
    }
}
