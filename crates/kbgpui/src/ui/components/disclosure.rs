use std::sync::Arc;

use gpui::ClickEvent;

use crate::ui::prelude::*;
use crate::ui::{Color, Icon, IconName, IconSize};

/// Expand/collapse control used by tree roots (Zed `Disclosure`).
#[derive(IntoElement)]
#[allow(clippy::type_complexity)]
pub struct Disclosure {
    id: ElementId,
    is_open: bool,
    opened_icon: IconName,
    closed_icon: IconName,
    on_toggle_expanded: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Disclosure {
    pub fn new(id: impl Into<ElementId>, is_open: bool) -> Self {
        Self {
            id: id.into(),
            is_open,
            opened_icon: IconName::ChevronDown,
            closed_icon: IconName::ChevronRight,
            on_toggle_expanded: None,
        }
    }

    pub fn opened_icon(mut self, icon: IconName) -> Self {
        self.opened_icon = icon;
        self
    }

    pub fn closed_icon(mut self, icon: IconName) -> Self {
        self.closed_icon = icon;
        self
    }

    pub fn on_toggle_expanded(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle_expanded = Some(Arc::new(handler));
        self
    }
}

impl RenderOnce for Disclosure {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon = if self.is_open {
            self.opened_icon
        } else {
            self.closed_icon
        };
        h_flex()
            .id(self.id)
            .cursor_pointer()
            .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
            .when_some(self.on_toggle_expanded, |this, on_toggle| {
                this.on_click(move |event, window, cx| {
                    cx.stop_propagation();
                    on_toggle(event, window, cx);
                })
            })
    }
}
