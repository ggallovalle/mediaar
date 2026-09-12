//! Theme tokens for kbgpui, mirroring Zed's `theme` crate surface.
//!
//! Palettes are Catppuccin Latte (light) and Mocha (dark).

use std::sync::Arc;

use catppuccin::{FlavorColors, PALETTE};
use gpui::{App, Global, Hsla, Rgba, transparent_black};

/// The appearance of the theme.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Appearance {
    /// A light appearance.
    Light,
    /// A dark appearance.
    Dark,
}

impl Appearance {
    /// Returns whether the appearance is light.
    pub fn is_light(&self) -> bool {
        match self {
            Self::Light => true,
            Self::Dark => false,
        }
    }
}

/// Semantic UI colors (Zed `ThemeColors`, fields used by `ui` components).
#[derive(Clone, Debug)]
pub struct ThemeColors {
    /// Border color. Used for most borders, is usually a high contrast color.
    pub border: Hsla,
    /// Border color. Used for focused elements, like keyboard focused list item.
    pub border_focused: Hsla,
    /// Border color. Used for transparent borders.
    pub border_transparent: Hsla,
    /// Background Color. Used for the disabled state of an element.
    pub element_disabled: Hsla,
    /// Text Color. Default text color used for most text.
    pub text: Hsla,
    /// Text Color. Color of muted or deemphasized text.
    pub text_muted: Hsla,
    /// Text Color. Color used for text denoting disabled elements.
    pub text_disabled: Hsla,
    /// Text Color. Color used for emphasis or highlighting certain text.
    pub text_accent: Hsla,
}

/// Status colors (Zed `StatusColors`, fields used by `ui` components).
#[derive(Clone, Debug)]
pub struct StatusColors {
    /// Represents informational status updates or messages.
    pub info: Hsla,
}

/// A theme is a collection of colors used to build a consistent appearance.
#[derive(Clone, Debug)]
pub struct Theme {
    /// Light vs dark chrome.
    pub appearance: Appearance,
    colors: ThemeColors,
    status: StatusColors,
}

impl Theme {
    /// Catppuccin Latte.
    pub fn latte() -> Self {
        Self::from_flavor(Appearance::Light, &PALETTE.latte.colors)
    }

    /// Catppuccin Mocha.
    pub fn mocha() -> Self {
        Self::from_flavor(Appearance::Dark, &PALETTE.mocha.colors)
    }

    fn from_flavor(appearance: Appearance, colors: &FlavorColors) -> Self {
        Self {
            appearance,
            colors: ThemeColors {
                border: ctp(&colors.surface1),
                border_focused: ctp(&colors.lavender),
                border_transparent: transparent_black(),
                element_disabled: ctp(&colors.surface1),
                text: ctp(&colors.text),
                text_muted: ctp(&colors.subtext0),
                text_disabled: ctp(&colors.overlay0),
                text_accent: ctp(&colors.mauve),
            },
            status: StatusColors {
                info: ctp(&colors.blue),
            },
        }
    }

    /// Semantic UI colors.
    pub fn colors(&self) -> &ThemeColors {
        &self.colors
    }

    /// Status colors.
    pub fn status(&self) -> &StatusColors {
        &self.status
    }
}

fn ctp(color: &catppuccin::Color) -> Hsla {
    Hsla::from(Rgba {
        r: f32::from(color.rgb.r) / 255.0,
        g: f32::from(color.rgb.g) / 255.0,
        b: f32::from(color.rgb.b) / 255.0,
        a: 1.0,
    })
}

struct GlobalTheme {
    theme: Arc<Theme>,
}

impl Global for GlobalTheme {}

/// Implementing this trait allows accessing the active theme.
pub trait ActiveTheme {
    /// Returns the active theme.
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        &self.global::<GlobalTheme>().theme
    }
}

/// Install the default dark theme (Mocha). Call once at app start.
pub fn init(cx: &mut App) {
    set_theme(cx, Theme::mocha());
}

/// Replace the active theme (e.g. Latte ↔ Mocha).
pub fn set_theme(cx: &mut App, theme: Theme) {
    cx.set_global(GlobalTheme {
        theme: Arc::new(theme),
    });
}
