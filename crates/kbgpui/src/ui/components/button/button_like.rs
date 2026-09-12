use gpui::{
    AnyElement, AnyView, ClickEvent, CursorStyle, DefiniteLength, FocusHandle, Hsla, MouseButton,
    StyleRefinement, relative, transparent_black,
};

use crate::theme::ActiveTheme;
use crate::ui::Color;
use crate::ui::prelude::*;
use crate::ui::styles::ElevationIndex;
use crate::ui::traits::clickable::Clickable;
use crate::ui::traits::disableable::Disableable;

/// A trait for buttons that can be Selected. Enables setting the [`ButtonStyle`] of a button when it is selected.
pub trait SelectableButton: Toggleable {
    fn selected_style(self, style: ButtonStyle) -> Self;
}

/// A common set of traits all buttons must implement.
pub trait ButtonCommon: Clickable + Disableable {
    fn id(&self) -> &ElementId;
    fn style(self, style: ButtonStyle) -> Self;
    fn size(self, size: ButtonSize) -> Self;
    fn tooltip(self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self;
    fn tab_index(self, tab_index: impl Into<isize>) -> Self;
    fn layer(self, elevation: ElevationIndex) -> Self;
    fn track_focus(self, focus_handle: &FocusHandle) -> Self;
}

/// Fixed-width helpers shared by button-like controls.
pub trait FixedWidth {
    fn width(self, width: impl Into<DefiniteLength>) -> Self;
    fn full_width(self) -> Self;
}

/// Where an icon sits relative to a label.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum IconPosition {
    #[default]
    Start,
    End,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum KeybindingPosition {
    Start,
    #[default]
    End,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum TintColor {
    #[default]
    Accent,
    Error,
    Warning,
    Success,
}

impl TintColor {
    fn button_like_style(self, cx: &mut App) -> ButtonLikeStyles {
        match self {
            TintColor::Accent => ButtonLikeStyles {
                background: cx.theme().status().info_background,
                border_color: cx.theme().status().info_border,
                label_color: cx.theme().colors().text,
                icon_color: cx.theme().colors().text,
            },
            TintColor::Error => ButtonLikeStyles {
                background: cx.theme().status().error_background,
                border_color: cx.theme().status().error_border,
                label_color: cx.theme().colors().text,
                icon_color: cx.theme().colors().text,
            },
            TintColor::Warning => ButtonLikeStyles {
                background: cx.theme().status().warning_background,
                border_color: cx.theme().status().warning_border,
                label_color: cx.theme().colors().text,
                icon_color: cx.theme().colors().text,
            },
            TintColor::Success => ButtonLikeStyles {
                background: cx.theme().status().success_background,
                border_color: cx.theme().status().success_border,
                label_color: cx.theme().colors().text,
                icon_color: cx.theme().colors().text,
            },
        }
    }
}

impl From<TintColor> for Color {
    fn from(tint: TintColor) -> Self {
        match tint {
            TintColor::Accent => Color::Accent,
            TintColor::Error => Color::Error,
            TintColor::Warning => Color::Warning,
            TintColor::Success => Color::Success,
        }
    }
}

impl From<ButtonStyle> for Color {
    fn from(style: ButtonStyle) -> Self {
        match style {
            ButtonStyle::Tinted(tint) => tint.into(),
            _ => Color::Default,
        }
    }
}

/// The visual appearance of a button.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ButtonStyle {
    Filled,
    Tinted(TintColor),
    Outlined,
    OutlinedGhost,
    OutlinedCustom(Hsla),
    #[default]
    Subtle,
    Transparent,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub(crate) struct ButtonLikeRounding {
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_right: bool,
    pub bottom_left: bool,
}

impl ButtonLikeRounding {
    pub const ALL: Self = Self {
        top_left: true,
        top_right: true,
        bottom_right: true,
        bottom_left: true,
    };
    pub const LEFT: Self = Self {
        top_left: true,
        top_right: false,
        bottom_right: false,
        bottom_left: true,
    };
    pub const RIGHT: Self = Self {
        top_left: false,
        top_right: true,
        bottom_right: true,
        bottom_left: false,
    };
}

#[derive(Debug, Clone)]
pub(crate) struct ButtonLikeStyles {
    pub background: Hsla,
    #[allow(unused)]
    pub border_color: Hsla,
    #[allow(unused)]
    pub label_color: Hsla,
    #[allow(unused)]
    pub icon_color: Hsla,
}

fn element_bg_from_elevation(elevation: Option<ElevationIndex>, cx: &mut App) -> Hsla {
    match elevation {
        Some(ElevationIndex::Background) => cx.theme().colors().element_background,
        Some(ElevationIndex::ElevatedSurface) => cx.theme().colors().elevated_surface_background,
        Some(ElevationIndex::Surface) => cx.theme().colors().surface_background,
        Some(ElevationIndex::ModalSurface) => cx.theme().colors().background,
        _ => cx.theme().colors().element_background,
    }
}

impl ButtonStyle {
    pub(crate) fn enabled(
        self,
        elevation: Option<ElevationIndex>,
        cx: &mut App,
    ) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: element_bg_from_elevation(elevation, cx),
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(cx),
            ButtonStyle::Outlined => ButtonLikeStyles {
                background: element_bg_from_elevation(elevation, cx),
                border_color: cx.theme().colors().border_variant,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::OutlinedGhost => ButtonLikeStyles {
                background: transparent_black(),
                border_color: cx.theme().colors().border_variant,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::OutlinedCustom(border_color) => ButtonLikeStyles {
                background: transparent_black(),
                border_color,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_background,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
        }
    }

    pub(crate) fn hovered(
        self,
        elevation: Option<ElevationIndex>,
        cx: &mut App,
    ) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => {
                let mut filled_background = element_bg_from_elevation(elevation, cx);
                filled_background.fade_out(0.5);
                ButtonLikeStyles {
                    background: filled_background,
                    border_color: transparent_black(),
                    label_color: Color::Default.color(cx),
                    icon_color: Color::Default.color(cx),
                }
            }
            ButtonStyle::Tinted(tint) => {
                let mut styles = tint.button_like_style(cx);
                styles.background.fade_out(0.12);
                styles
            }
            ButtonStyle::Outlined => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_hover,
                border_color: cx.theme().colors().border,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::OutlinedGhost => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_hover,
                border_color: cx.theme().colors().border,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::OutlinedCustom(border_color) => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_hover,
                border_color,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_hover,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: Color::Muted.color(cx),
                icon_color: Color::Muted.color(cx),
            },
        }
    }

    pub(crate) fn active(self, cx: &mut App) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_active,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(cx),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_active,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Outlined => ButtonLikeStyles {
                background: cx.theme().colors().element_active,
                border_color: cx.theme().colors().border_variant,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::OutlinedGhost => ButtonLikeStyles {
                background: transparent_black(),
                border_color: cx.theme().colors().border_variant,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::OutlinedCustom(border_color) => ButtonLikeStyles {
                background: cx.theme().colors().element_active,
                border_color,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: Color::Muted.color(cx),
                icon_color: Color::Muted.color(cx),
            },
        }
    }

    #[allow(unused)]
    pub(crate) fn disabled(
        self,
        _elevation: Option<ElevationIndex>,
        _window: &mut Window,
        cx: &mut App,
    ) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(cx),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
            ButtonStyle::Outlined => ButtonLikeStyles {
                background: cx.theme().colors().element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::OutlinedGhost => ButtonLikeStyles {
                background: transparent_black(),
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::OutlinedCustom(_) => ButtonLikeStyles {
                background: cx.theme().colors().element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
        }
    }
}

/// The height of a button.
#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Large,
    Medium,
    #[default]
    Default,
    Compact,
    None,
}

impl ButtonSize {
    pub fn rems(self) -> Rems {
        match self {
            ButtonSize::Large => rems_from_px(32.0_f32),
            ButtonSize::Medium => rems_from_px(28.0_f32),
            ButtonSize::Default => rems_from_px(22.0_f32),
            ButtonSize::Compact => rems_from_px(18.0_f32),
            ButtonSize::None => rems_from_px(16.0_f32),
        }
    }
}

/// Unconstrained button chrome. Prefer [`super::Button`] at call sites.
#[derive(IntoElement)]
#[allow(clippy::type_complexity)]
pub struct ButtonLike {
    pub(super) base: Div,
    id: ElementId,
    pub(super) style: ButtonStyle,
    pub(super) disabled: bool,
    pub(super) selected: bool,
    pub(super) selected_style: Option<ButtonStyle>,
    pub(super) width: Option<DefiniteLength>,
    pub(super) height: Option<DefiniteLength>,
    pub(super) layer: Option<ElevationIndex>,
    tab_index: Option<isize>,
    size: ButtonSize,
    rounding: Option<ButtonLikeRounding>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    cursor_style: CursorStyle,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
    focus_handle: Option<FocusHandle>,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            style: ButtonStyle::default(),
            disabled: false,
            selected: false,
            selected_style: None,
            width: None,
            height: None,
            size: ButtonSize::Default,
            rounding: Some(ButtonLikeRounding::ALL),
            tooltip: None,
            children: Vec::new(),
            cursor_style: CursorStyle::PointingHand,
            on_click: None,
            layer: None,
            tab_index: None,
            focus_handle: None,
        }
    }

    pub fn new_rounded_left(id: impl Into<ElementId>) -> Self {
        Self::new(id).rounding(ButtonLikeRounding::LEFT)
    }

    pub fn new_rounded_right(id: impl Into<ElementId>) -> Self {
        Self::new(id).rounding(ButtonLikeRounding::RIGHT)
    }

    pub fn new_rounded_all(id: impl Into<ElementId>) -> Self {
        Self::new(id).rounding(ButtonLikeRounding::ALL)
    }

    pub fn height(mut self, height: DefiniteLength) -> Self {
        self.height = Some(height);
        self
    }

    pub(crate) fn rounding(mut self, rounding: impl Into<Option<ButtonLikeRounding>>) -> Self {
        self.rounding = rounding.into();
        self
    }
}

impl Disableable for ButtonLike {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for ButtonLike {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl SelectableButton for ButtonLike {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.selected_style = Some(style);
        self
    }
}

impl Clickable for ButtonLike {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl FixedWidth for ButtonLike {
    fn width(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.width = Some(width.into());
        self
    }

    fn full_width(mut self) -> Self {
        self.width = Some(relative(1.0_f32));
        self
    }
}

impl ButtonCommon for ButtonLike {
    fn id(&self) -> &ElementId {
        &self.id
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    fn tab_index(mut self, tab_index: impl Into<isize>) -> Self {
        self.tab_index = Some(tab_index.into());
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.layer = Some(elevation);
        self
    }

    fn track_focus(mut self, focus_handle: &gpui::FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle.clone());
        self
    }
}

impl ParentElement for ButtonLike {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ButtonLike {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = self
            .selected_style
            .filter(|_| self.selected)
            .unwrap_or(self.style);

        let is_outlined = matches!(
            self.style,
            ButtonStyle::Outlined | ButtonStyle::OutlinedGhost | ButtonStyle::OutlinedCustom(_)
        );

        self.base
            .h_flex()
            .id(self.id.clone())
            .when_some(self.tab_index, |this, tab_index| this.tab_index(tab_index))
            .when_some(self.focus_handle, |this, focus_handle| {
                this.track_focus(&focus_handle)
            })
            .text_ui(cx)
            .group("")
            .flex_none()
            .h(self.height.unwrap_or(self.size.rems().into()))
            .when_some(self.width, |this, width| {
                this.w(width).justify_center().text_center()
            })
            .when(is_outlined, |this| this.border_1())
            .when(self.rounding.is_some(), |this| this.rounded_md())
            .gap(DynamicSpacing::Base04.rems(cx))
            .map(|this| match self.size {
                ButtonSize::Large | ButtonSize::Medium => this.px(DynamicSpacing::Base08.rems(cx)),
                ButtonSize::Default | ButtonSize::Compact => {
                    this.px(DynamicSpacing::Base04.rems(cx))
                }
                ButtonSize::None => this.px(px(1.0_f32)),
            })
            .border_color(style.enabled(self.layer, cx).border_color)
            .bg(style.enabled(self.layer, cx).background)
            .when(self.disabled, |this| {
                if self.cursor_style == CursorStyle::PointingHand {
                    this.cursor(CursorStyle::OperationNotAllowed)
                } else {
                    this.cursor(self.cursor_style)
                }
            })
            .when(!self.disabled, |this| {
                let hovered_style = style.hovered(self.layer, cx);
                let focus_color =
                    |refinement: StyleRefinement| refinement.bg(hovered_style.background);

                this.cursor(self.cursor_style)
                    .hover(focus_color)
                    .active(|active| active.bg(style.active(cx).background))
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            (on_click)(event, window, cx)
                        })
                },
            )
            .children(self.children)
    }
}
