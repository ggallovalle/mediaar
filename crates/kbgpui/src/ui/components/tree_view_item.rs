use std::sync::Arc;

use gpui::ClickEvent;

use crate::theme::ActiveTheme;
use crate::ui::prelude::*;
use crate::ui::traits::disableable::Disableable;
use crate::ui::{Color, Disclosure, IconName, Label, LabelCommon};

/// Hierarchical nav row (Zed `TreeViewItem`).
#[derive(IntoElement)]
#[allow(clippy::type_complexity)]
pub struct TreeViewItem {
    id: ElementId,
    label: SharedString,
    expanded: bool,
    selected: bool,
    disabled: bool,
    root_item: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TreeViewItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            expanded: false,
            selected: false,
            disabled: false,
            root_item: false,
            on_click: None,
            on_toggle: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn expanded(mut self, toggle: bool) -> Self {
        self.expanded = toggle;
        self
    }

    pub fn on_toggle(
        mut self,
        on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Arc::new(on_toggle));
        self
    }

    pub fn root_item(mut self, root_item: bool) -> Self {
        self.root_item = root_item;
        self
    }
}

impl Disableable for TreeViewItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for TreeViewItem {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for TreeViewItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let selected_bg = cx.theme().colors().element_active.opacity(0.5);
        let transparent_border = cx.theme().colors().border.opacity(0.);
        let selected_border = cx.theme().colors().border.opacity(0.4);
        let hover_bg = cx.theme().colors().element_hover;
        let indent_line = cx.theme().colors().border.opacity(0.5);
        let item_size = rems_from_px(28.0_f32);
        let root_item = self.root_item;
        let expanded = self.expanded;
        let selected = self.selected;
        let label = self.label.clone();

        h_flex().id(self.id).w_full().child(
            h_flex()
                .id("inner_tree_view_item")
                .cursor_pointer()
                .size_full()
                .h(item_size)
                .pl_0p5()
                .pr_1()
                .gap_2()
                .rounded_sm()
                .border_1()
                .border_color(transparent_border)
                .when(selected, |this| {
                    this.border_color(selected_border).bg(selected_bg)
                })
                .hover(move |s| s.bg(hover_bg))
                .map(|this| {
                    if root_item {
                        this.child(
                            Disclosure::new("toggle", expanded)
                                .opened_icon(IconName::ChevronDown)
                                .closed_icon(IconName::ChevronRight)
                                .when_some(self.on_toggle.clone(), |disclosure, on_toggle| {
                                    disclosure.on_toggle_expanded(move |e, w, cx| {
                                        on_toggle(e, w, cx);
                                    })
                                }),
                        )
                        .child(Label::new(label).when(!selected, |this| this.color(Color::Muted)))
                    } else {
                        this.child(
                            h_flex()
                                .h(item_size)
                                .w(px(22.))
                                .flex_none()
                                .justify_center()
                                .child(div().w_px().h_full().bg(indent_line)),
                        )
                        .child(Label::new(label).when(!selected, |this| this.color(Color::Muted)))
                    }
                })
                .when_some(
                    self.on_click.filter(|_| !self.disabled),
                    |this, on_click| this.on_click(on_click),
                ),
        )
    }
}
