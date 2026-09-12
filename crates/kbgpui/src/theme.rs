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
    /// App / window background.
    pub background: Hsla,
    /// Grounded surfaces (sidebar, panels).
    pub surface_background: Hsla,
    /// Elevated surfaces (menus, dropdowns).
    pub elevated_surface_background: Hsla,
    /// Hover fill for controls that sit on a surface.
    pub element_hover: Hsla,
    /// Selected fill for nav rows and menu entries.
    pub element_selected: Hsla,
    /// Pressed / active fill.
    pub element_active: Hsla,
    /// Background Color. Used for the disabled state of an element.
    pub element_disabled: Hsla,
    /// Default fill for outlined / filled buttons.
    pub element_background: Hsla,
    /// Panel / sidebar background.
    pub panel_background: Hsla,
    /// Editor / search field background.
    pub editor_background: Hsla,
    /// Lower-contrast border (outlined buttons).
    pub border_variant: Hsla,
    /// Border color for disabled controls.
    pub border_disabled: Hsla,
    /// Transparent fill for ghost / outlined controls on a surface.
    pub ghost_element_background: Hsla,
    /// Hover fill for ghost list rows and menus.
    pub ghost_element_hover: Hsla,
    /// Pressed fill for ghost list rows.
    pub ghost_element_active: Hsla,
    /// Selected fill for ghost list rows (keyboard highlight).
    pub ghost_element_selected: Hsla,
    /// Disabled fill for ghost / subtle controls.
    pub ghost_element_disabled: Hsla,
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
    pub info_background: Hsla,
    pub info_border: Hsla,
    pub error: Hsla,
    pub error_background: Hsla,
    pub error_border: Hsla,
    pub warning: Hsla,
    pub warning_background: Hsla,
    pub warning_border: Hsla,
    pub success: Hsla,
    pub success_background: Hsla,
    pub success_border: Hsla,
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
                background: ctp(&colors.base),
                surface_background: ctp(&colors.mantle),
                elevated_surface_background: ctp(&colors.surface0),
                element_hover: ctp(&colors.surface1),
                element_selected: ctp(&colors.surface1),
                element_active: ctp(&colors.surface1),
                element_disabled: ctp(&colors.surface1),
                element_background: ctp(&colors.surface0),
                panel_background: ctp(&colors.mantle),
                editor_background: ctp(&colors.base),
                border_variant: ctp(&colors.surface1),
                border_disabled: ctp(&colors.overlay0),
                ghost_element_background: transparent_black(),
                ghost_element_hover: ctp(&colors.surface1).opacity(0.6),
                ghost_element_active: ctp(&colors.surface1),
                ghost_element_selected: ctp(&colors.surface1),
                ghost_element_disabled: ctp(&colors.surface1).opacity(0.4),
                text: ctp(&colors.text),
                text_muted: ctp(&colors.subtext0),
                text_disabled: ctp(&colors.overlay0),
                text_accent: ctp(&colors.mauve),
            },
            status: StatusColors {
                info: ctp(&colors.blue),
                info_background: ctp(&colors.blue).opacity(0.2),
                info_border: ctp(&colors.blue),
                error: ctp(&colors.red),
                error_background: ctp(&colors.red).opacity(0.2),
                error_border: ctp(&colors.red),
                warning: ctp(&colors.yellow),
                warning_background: ctp(&colors.yellow).opacity(0.2),
                warning_border: ctp(&colors.yellow),
                success: ctp(&colors.green),
                success_background: ctp(&colors.green).opacity(0.2),
                success_border: ctp(&colors.green),
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
