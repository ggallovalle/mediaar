use gpui::{App, BoxShadow, Styled, hsla, point, px};

use crate::theme::ActiveTheme;

/// Extends [`gpui::Styled`] with Zed-style stacking helpers.
pub trait StyledExt: Styled + Sized {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    ///
    /// Sets `flex()`, `flex_col()`
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    /// Elevated surface used by menus and dropdowns (Zed `elevation_2`).
    fn elevation_2(self, cx: &App) -> Self {
        self.bg(cx.theme().colors().elevated_surface_background)
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .shadow(vec![
                BoxShadow {
                    color: hsla(0., 0., 0., 0.12),
                    offset: point(px(0.), px(2.)),
                    blur_radius: px(3.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., 0.06),
                    offset: point(px(0.), px(1.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                },
            ])
    }
}

impl<E: Styled> StyledExt for E {}
