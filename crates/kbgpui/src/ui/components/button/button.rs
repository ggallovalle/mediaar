use gpui::{AnyView, DefiniteLength};

use crate::ui::prelude::*;
use crate::ui::styles::ElevationIndex;
use crate::ui::traits::clickable::Clickable;
use crate::ui::traits::disableable::Disableable;
use crate::ui::{Color, Icon, Label, LabelCommon, LabelSize};

use super::{ButtonCommon, ButtonLike, ButtonSize, ButtonStyle, FixedWidth, SelectableButton};

/// A labeled button with optional start/end icons (Zed `Button`).
#[derive(IntoElement)]
pub struct Button {
    base: ButtonLike,
    label: SharedString,
    label_color: Option<Color>,
    label_size: Option<LabelSize>,
    selected_label: Option<SharedString>,
    selected_label_color: Option<Color>,
    start_icon: Option<Icon>,
    end_icon: Option<Icon>,
    alpha: Option<f32>,
    truncate: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: ButtonLike::new(id),
            label: label.into(),
            label_color: None,
            label_size: None,
            selected_label: None,
            selected_label_color: None,
            start_icon: None,
            end_icon: None,
            alpha: None,
            truncate: false,
        }
    }

    pub fn color(mut self, label_color: impl Into<Option<Color>>) -> Self {
        self.label_color = label_color.into();
        self
    }

    pub fn label_size(mut self, label_size: impl Into<Option<LabelSize>>) -> Self {
        self.label_size = label_size.into();
        self
    }

    pub fn selected_label<L: Into<SharedString>>(mut self, label: impl Into<Option<L>>) -> Self {
        self.selected_label = label.into().map(Into::into);
        self
    }

    pub fn selected_label_color(mut self, color: impl Into<Option<Color>>) -> Self {
        self.selected_label_color = color.into();
        self
    }

    pub fn start_icon(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.start_icon = icon.into();
        self
    }

    pub fn end_icon(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.end_icon = icon.into();
        self
    }

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
}

impl Toggleable for Button {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.base = self.base.toggle_state(selected);
        self
    }
}

impl SelectableButton for Button {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.selected_style(style);
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Clickable for Button {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
        self
    }
}

impl FixedWidth for Button {
    fn width(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.base = self.base.width(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.full_width();
        self
    }
}

impl ButtonCommon for Button {
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.style(style);
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }

    fn tab_index(mut self, tab_index: impl Into<isize>) -> Self {
        self.base = self.base.tab_index(tab_index);
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.base = self.base.layer(elevation);
        self
    }

    fn track_focus(mut self, focus_handle: &gpui::FocusHandle) -> Self {
        self.base = self.base.track_focus(focus_handle);
        self
    }
}

impl RenderOnce for Button {
    #[allow(refining_impl_trait)]
    fn render(self, _window: &mut Window, _cx: &mut App) -> ButtonLike {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;

        let label = self
            .selected_label
            .filter(|_| is_selected)
            .unwrap_or(self.label);

        let label_color = if is_disabled {
            Color::Disabled
        } else if is_selected {
            self.selected_label_color
                .or(self.label_color)
                .unwrap_or_default()
        } else {
            self.label_color.unwrap_or_default()
        };

        self.base.child(
            h_flex()
                .when(self.truncate, |this| this.min_w_0().overflow_hidden())
                .gap(DynamicSpacing::Base04.rems(_cx))
                .when_some(self.start_icon, |this, icon| {
                    this.child(if is_disabled {
                        icon.color(Color::Disabled)
                    } else {
                        icon
                    })
                })
                .child(
                    h_flex()
                        .when(self.truncate, |this| this.min_w_0().overflow_hidden())
                        .gap(DynamicSpacing::Base06.rems(_cx))
                        .justify_between()
                        .child(
                            Label::new(label)
                                .color(label_color)
                                .size(self.label_size.unwrap_or_default())
                                .when_some(self.alpha, |this, alpha| this.alpha(alpha))
                                .when(self.truncate, |this| this.truncate()),
                        ),
                )
                .when_some(self.end_icon, |this, icon| {
                    this.child(if is_disabled {
                        icon.color(Color::Disabled)
                    } else {
                        icon
                    })
                }),
        )
    }
}
