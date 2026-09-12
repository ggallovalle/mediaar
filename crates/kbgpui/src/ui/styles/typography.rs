use gpui::{App, Pixels, Rems, Styled, px};

use super::units::rems_from_px;

/// Extends [`gpui::Styled`] with typography-related styling methods.
pub trait StyledTypography: Styled + Sized {
    /// The large size for UI text.
    ///
    /// `1rem` or `16px` at the default scale of `1rem` = `16px`.
    fn text_ui_lg(self, cx: &App) -> Self {
        self.text_size(TextSize::Large.rems(cx))
    }

    /// The default size for UI text.
    ///
    /// `0.825rem` or `14px` at the default scale of `1rem` = `16px`.
    fn text_ui(self, cx: &App) -> Self {
        self.text_size(TextSize::default().rems(cx))
    }

    /// The small size for UI text.
    ///
    /// `0.75rem` or `12px` at the default scale of `1rem` = `16px`.
    fn text_ui_sm(self, cx: &App) -> Self {
        self.text_size(TextSize::Small.rems(cx))
    }

    /// The extra small size for UI text.
    ///
    /// `0.625rem` or `10px` at the default scale of `1rem` = `16px`.
    fn text_ui_xs(self, cx: &App) -> Self {
        self.text_size(TextSize::XSmall.rems(cx))
    }
}

impl<E: Styled> StyledTypography for E {}

/// A utility for getting the size of various semantic text sizes.
#[derive(Debug, Default, Clone)]
pub enum TextSize {
    /// The default size for UI text.
    #[default]
    Default,
    /// The large size for UI text.
    Large,
    /// The small size for UI text.
    Small,
    /// The extra small size for UI text.
    XSmall,
}

impl TextSize {
    /// Returns the text size in rems.
    pub fn rems(self, _cx: &App) -> Rems {
        match self {
            Self::Large => rems_from_px(16.0_f32),
            Self::Default => rems_from_px(14.0_f32),
            Self::Small => rems_from_px(12.0_f32),
            Self::XSmall => rems_from_px(10.0_f32),
        }
    }

    /// Returns the text size in pixels.
    pub fn pixels(self, _cx: &App) -> Pixels {
        match self {
            Self::Large => px(16.0),
            Self::Default => px(14.0),
            Self::Small => px(12.0),
            Self::XSmall => px(10.0),
        }
    }
}
