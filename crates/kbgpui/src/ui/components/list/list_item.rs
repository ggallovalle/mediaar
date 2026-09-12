use gpui::{AnyElement, ClickEvent, MouseButton, MouseDownEvent, px};

use crate::theme::ActiveTheme;
use crate::ui::prelude::*;
use crate::ui::traits::disableable::Disableable;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ListItemSpacing {
    #[default]
    Dense,
    ExtraDense,
    Sparse,
}

/// A row in a list or menu (Zed `ListItem`).
#[derive(IntoElement)]
#[allow(clippy::type_complexity)]
pub struct ListItem {
    id: ElementId,
    group_name: Option<SharedString>,
    disabled: bool,
    selected: bool,
    spacing: ListItemSpacing,
    indent_level: usize,
    indent_step_size: Pixels,
    inset: bool,
    selectable: bool,
    outlined: bool,
    rounded: bool,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    children: Vec<AnyElement>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_hover: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_secondary_mouse_down: Option<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
}

impl ListItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            group_name: None,
            disabled: false,
            selected: false,
            spacing: ListItemSpacing::Dense,
            indent_level: 0,
            indent_step_size: px(12.0_f32),
            inset: false,
            selectable: true,
            outlined: false,
            rounded: false,
            start_slot: None,
            end_slot: None,
            children: Vec::new(),
            on_click: None,
            on_hover: None,
            on_secondary_mouse_down: None,
        }
    }

    pub fn group_name(mut self, group_name: impl Into<SharedString>) -> Self {
        self.group_name = Some(group_name.into());
        self
    }

    pub fn spacing(mut self, spacing: ListItemSpacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn selectable(mut self, has_hover: bool) -> Self {
        self.selectable = has_hover;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn on_hover(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_hover = Some(Box::new(handler));
        self
    }

    pub fn on_secondary_mouse_down(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_secondary_mouse_down = Some(Box::new(handler));
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn indent_level(mut self, indent_level: usize) -> Self {
        self.indent_level = indent_level;
        self
    }

    pub fn indent_step_size(mut self, indent_step_size: Pixels) -> Self {
        self.indent_step_size = indent_step_size;
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, start_slot: impl Into<Option<E>>) -> Self {
        self.start_slot = start_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, end_slot: impl Into<Option<E>>) -> Self {
        self.end_slot = end_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn outlined(mut self) -> Self {
        self.outlined = true;
        self
    }

    pub fn rounded(mut self) -> Self {
        self.rounded = true;
        self
    }
}

impl Disableable for ListItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for ListItem {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for ListItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .when_some(self.group_name, |this, group| this.group(group))
            .w_full()
            .relative()
            .when(self.inset, |this| {
                this.ml(self.indent_level as f32 * self.indent_step_size)
                    .px(DynamicSpacing::Base04.rems(cx))
            })
            .when(!self.inset, |this| {
                this.when(self.selectable && !self.disabled, |this| {
                    this.hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                        .active(|style| style.bg(cx.theme().colors().ghost_element_active))
                        .when(self.outlined, |this| this.rounded_sm())
                        .when(self.selected, |this| {
                            this.bg(cx.theme().colors().ghost_element_selected)
                        })
                })
            })
            .when(self.rounded, |this| this.rounded_sm())
            .when_some(self.on_hover, |this, on_hover| this.on_hover(on_hover))
            .child(
                h_flex()
                    .id("inner_list_item")
                    .group("list_item")
                    .w_full()
                    .relative()
                    .gap_1()
                    .px(DynamicSpacing::Base06.rems(cx))
                    .map(|this| match self.spacing {
                        ListItemSpacing::Dense => this,
                        ListItemSpacing::ExtraDense => this,
                        ListItemSpacing::Sparse => this.py_1(),
                    })
                    .when(self.inset, |this| {
                        this.when(self.selectable && !self.disabled, |this| {
                            this.hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                                .active(|style| style.bg(cx.theme().colors().ghost_element_active))
                                .when(self.selected, |this| {
                                    this.bg(cx.theme().colors().ghost_element_selected)
                                })
                        })
                    })
                    .when_some(
                        self.on_click.filter(|_| !self.disabled),
                        |this, on_click| this.cursor_pointer().on_click(on_click),
                    )
                    .when(self.outlined, |this| {
                        this.border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_sm()
                            .overflow_hidden()
                    })
                    .when_some(self.on_secondary_mouse_down, |this, on_mouse_down| {
                        this.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                            (on_mouse_down)(event, window, cx)
                        })
                    })
                    .map(|this| {
                        if self.inset {
                            this.rounded_sm()
                        } else {
                            this.ml(self.indent_level as f32 * self.indent_step_size)
                        }
                    })
                    .when_some(self.start_slot, |this, slot| this.child(slot))
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w_0()
                            .gap(DynamicSpacing::Base08.rems(cx))
                            .overflow_hidden()
                            .children(self.children),
                    )
                    .when_some(self.end_slot, |this, end_slot| {
                        this.justify_between().child(
                            h_flex()
                                .flex_shrink_0()
                                .gap(DynamicSpacing::Base02.rems(cx))
                                .child(end_slot),
                        )
                    }),
            )
    }
}
