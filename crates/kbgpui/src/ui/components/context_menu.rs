use std::rc::Rc;

use gpui::{
    Action, App, ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    KeyBinding, Subscription, actions,
};

use crate::theme::ActiveTheme;
use crate::ui::prelude::*;
use crate::ui::{
    Color, Icon, IconName, IconPosition, IconSize, Label, LabelCommon, List, ListItem,
};

actions!(
    menu,
    [
        Cancel,
        Confirm,
        SelectPrevious,
        SelectNext,
        SelectFirst,
        SelectLast,
    ]
);

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", SelectPrevious, Some("menu")),
        KeyBinding::new("down", SelectNext, Some("menu")),
        KeyBinding::new("home", SelectFirst, Some("menu")),
        KeyBinding::new("end", SelectLast, Some("menu")),
        KeyBinding::new("enter", Confirm, Some("menu")),
        KeyBinding::new("escape", Cancel, Some("menu")),
    ]);
}

#[allow(clippy::type_complexity)]
pub struct ContextMenuEntry {
    toggle: Option<(IconPosition, bool)>,
    label: SharedString,
    handler: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    disabled: bool,
}

pub enum ContextMenuItem {
    Separator,
    Label(SharedString),
    Entry(ContextMenuEntry),
}

impl ContextMenuItem {
    fn is_selectable(&self) -> bool {
        match self {
            ContextMenuItem::Separator | ContextMenuItem::Label(_) => false,
            ContextMenuItem::Entry(ContextMenuEntry { disabled, .. }) => !*disabled,
        }
    }
}

/// A list of actions shown from a [`crate::ui::DropdownMenu`] (Zed `ContextMenu`).
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    focus_handle: FocusHandle,
    selected_index: Option<usize>,
    key_context: SharedString,
    pub(crate) clicked: bool,
    _on_blur_subscription: Subscription,
}

impl ContextMenu {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        f: impl FnOnce(Self, &mut Window, &mut Context<Self>) -> Self,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let on_blur_subscription = cx.on_blur(&focus_handle, window, |this, window, cx| {
            this.cancel(&Cancel, window, cx);
        });
        f(
            Self {
                items: Vec::new(),
                focus_handle,
                selected_index: None,
                key_context: "menu".into(),
                clicked: false,
                _on_blur_subscription: on_blur_subscription,
            },
            window,
            cx,
        )
    }

    pub fn build(
        window: &mut Window,
        cx: &mut App,
        f: impl FnOnce(Self, &mut Window, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, f))
    }

    pub fn key_context(mut self, context: impl Into<SharedString>) -> Self {
        self.key_context = context.into();
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(ContextMenuItem::Separator);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.items.push(ContextMenuItem::Label(label.into()));
        self
    }

    pub fn entry(
        mut self,
        label: impl Into<SharedString>,
        _action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            handler: Rc::new(handler),
            disabled: false,
        }));
        self
    }

    pub fn toggleable_entry(
        self,
        label: impl Into<SharedString>,
        toggled: bool,
        position: IconPosition,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.toggleable_entry_disabled_when(label, toggled, false, position, action, handler)
    }

    pub fn toggleable_entry_disabled_when(
        mut self,
        label: impl Into<SharedString>,
        toggled: bool,
        disabled: bool,
        position: IconPosition,
        _action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: Some((position, toggled)),
            label: label.into(),
            handler: Rc::new(handler),
            disabled,
        }));
        self
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ix) = self.selected_index else {
            return;
        };

        if let Some(ContextMenuItem::Entry(ContextMenuEntry {
            handler,
            disabled: false,
            ..
        })) = self.items.get(ix)
        {
            (handler)(window, cx);
        }

        self.clicked = true;
        cx.emit(DismissEvent);
    }

    pub fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    pub fn clear_selected(&mut self) {
        self.selected_index = None;
    }

    pub fn select_first(&mut self, _: &SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.items.iter().position(|item| item.is_selectable()) {
            self.select_index(ix, window, cx);
        }
        cx.notify();
    }

    pub fn select_toggled_or_first(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let toggled_ix = self.items.iter().position(|item| {
            matches!(
                item,
                ContextMenuItem::Entry(ContextMenuEntry {
                    toggle: Some((_, true)),
                    ..
                })
            )
        });
        if let Some(ix) = toggled_ix {
            self.select_index(ix, window, cx);
            cx.notify();
        } else {
            self.select_first(&SelectFirst, window, cx);
        }
    }

    pub fn select_last(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<usize> {
        for (ix, item) in self.items.iter().enumerate().rev() {
            if item.is_selectable() {
                return self.select_index(ix, window, cx);
            }
        }
        None
    }

    fn handle_select_last(&mut self, _: &SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        if self.select_last(window, cx).is_some() {
            cx.notify();
        }
    }

    pub fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.selected_index {
            let next_index = ix + 1;
            if self.items.len() <= next_index {
                self.select_first(&SelectFirst, window, cx);
                return;
            }
            for (ix, item) in self.items.iter().enumerate().skip(next_index) {
                if item.is_selectable() {
                    self.select_index(ix, window, cx);
                    cx.notify();
                    return;
                }
            }
        }
        self.select_first(&SelectFirst, window, cx);
    }

    pub fn select_previous(
        &mut self,
        _: &SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(ix) = self.selected_index {
            for (ix, item) in self.items.iter().enumerate().take(ix).rev() {
                if item.is_selectable() {
                    self.select_index(ix, window, cx);
                    cx.notify();
                    return;
                }
            }
        }
        self.handle_select_last(&SelectLast, window, cx);
    }

    fn select_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let item = self.items.get(ix)?;
        if item.is_selectable() {
            self.selected_index = Some(ix);
        }
        Some(ix)
    }
}

impl EventEmitter<DismissEvent> for ContextMenu {}

impl Focusable for ContextMenu {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ContextMenu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items: Vec<_> = self
            .items
            .iter()
            .enumerate()
            .map(|(ix, item)| self.render_menu_item(ix, item, cx).into_any_element())
            .collect();

        v_flex()
            .id("context-menu")
            .key_context(self.key_context.as_ref())
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(ContextMenu::select_first))
            .on_action(cx.listener(ContextMenu::handle_select_last))
            .on_action(cx.listener(ContextMenu::select_next))
            .on_action(cx.listener(ContextMenu::select_previous))
            .on_action(cx.listener(ContextMenu::confirm))
            .on_action(cx.listener(ContextMenu::cancel))
            .elevation_2(cx)
            .min_w(px(200.0_f32))
            .flex_shrink_0()
            .occlude()
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                this.cancel(&Cancel, window, cx);
            }))
            .child(List::new().children(items))
    }
}

impl ContextMenu {
    fn render_menu_item(
        &self,
        ix: usize,
        item: &ContextMenuItem,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match item {
            ContextMenuItem::Separator => div()
                .h_px()
                .bg(cx.theme().colors().border)
                .mx(DynamicSpacing::Base06.rems(cx))
                .my_0p5()
                .into_any_element(),
            ContextMenuItem::Label(label) => ListItem::new(ix)
                .inset(true)
                .disabled(true)
                .selectable(false)
                .child(Label::new(label.clone()).color(Color::Muted))
                .into_any_element(),
            ContextMenuItem::Entry(entry) => {
                self.render_menu_entry(ix, entry, cx).into_any_element()
            }
        }
    }

    fn render_menu_entry(
        &self,
        ix: usize,
        entry: &ContextMenuEntry,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let handler = entry.handler.clone();
        let disabled = entry.disabled;
        let label = entry.label.clone();
        let toggle = entry.toggle;
        let icon_color = if disabled {
            Color::Muted
        } else if toggle.is_some() {
            Color::Accent
        } else {
            Color::Default
        };
        let label_color = if disabled {
            Color::Disabled
        } else {
            Color::Default
        };

        let check = toggle.map(|(position, toggled)| {
            (
                position,
                div()
                    .flex_none()
                    .when(!toggled, |this| this.opacity(0.))
                    .child(
                        Icon::new(IconName::Check)
                            .color(icon_color)
                            .size(IconSize::Small),
                    ),
            )
        });

        ListItem::new(ix)
            .group_name("label_container")
            .inset(true)
            .disabled(disabled)
            .toggle_state(Some(ix) == self.selected_index)
            .when(!disabled, |item| {
                item.on_hover(cx.listener(move |this, hovered, window, cx| {
                    if *hovered {
                        this.select_index(ix, window, cx);
                        window.focus(&this.focus_handle);
                        cx.notify();
                    }
                }))
            })
            .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                if disabled {
                    return;
                }
                (handler)(window, cx);
                this.clicked = true;
                cx.emit(DismissEvent);
                cx.notify();
            }))
            .map(|item| match check {
                Some((IconPosition::Start, check)) => item.start_slot(check),
                Some((IconPosition::End, check)) => item.end_slot(check),
                None => item,
            })
            .child(Label::new(label).color(label_color).truncate())
    }
}
