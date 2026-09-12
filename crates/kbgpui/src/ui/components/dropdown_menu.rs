use gpui::{Anchor, Entity, Pixels, Point};

use crate::ui::prelude::*;
use crate::ui::traits::disableable::Disableable;
use crate::ui::{
    Button, ButtonSize, ButtonStyle, Color, ContextMenu, Icon, IconName, IconSize, PopoverMenu,
    PopoverMenuHandle,
};

/// Visual style of a [`DropdownMenu`] trigger.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DropdownStyle {
    #[default]
    Solid,
    Outlined,
    Subtle,
    Ghost,
}

/// Combobox trigger + [`ContextMenu`] (Zed `DropdownMenu`).
#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    label: SharedString,
    trigger_size: ButtonSize,
    trigger_icon: Option<IconName>,
    style: DropdownStyle,
    menu: Entity<ContextMenu>,
    full_width: bool,
    disabled: bool,
    handle: Option<PopoverMenuHandle<ContextMenu>>,
    attach: Option<Anchor>,
    offset: Option<Point<Pixels>>,
    chevron: bool,
}

impl DropdownMenu {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        menu: Entity<ContextMenu>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            trigger_size: ButtonSize::Default,
            trigger_icon: Some(IconName::ChevronUpDown),
            style: DropdownStyle::default(),
            menu,
            full_width: false,
            disabled: false,
            handle: None,
            attach: None,
            offset: None,
            chevron: true,
        }
    }

    pub fn style(mut self, style: DropdownStyle) -> Self {
        self.style = style;
        self
    }

    pub fn trigger_size(mut self, size: ButtonSize) -> Self {
        self.trigger_size = size;
        self
    }

    pub fn trigger_icon(mut self, icon: IconName) -> Self {
        self.trigger_icon = Some(icon);
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn handle(mut self, handle: PopoverMenuHandle<ContextMenu>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn attach(mut self, attach: Anchor) -> Self {
        self.attach = Some(attach);
        self
    }

    pub fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn no_chevron(mut self) -> Self {
        self.chevron = false;
        self
    }
}

impl Disableable for DropdownMenu {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let button_style = match self.style {
            DropdownStyle::Solid => ButtonStyle::Filled,
            DropdownStyle::Subtle => ButtonStyle::Subtle,
            DropdownStyle::Outlined => ButtonStyle::Outlined,
            DropdownStyle::Ghost => ButtonStyle::Transparent,
        };

        let handle = self.handle.unwrap_or_default();

        let text_button = Button::new(self.id.clone(), self.label.clone())
            .style(button_style)
            .when_some(self.trigger_icon.filter(|_| self.chevron), |this, icon| {
                this.end_icon(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted))
            })
            .size(self.trigger_size)
            .disabled(self.disabled);

        let menu_for_open = self.menu.clone();
        PopoverMenu::new((self.id.clone(), "popover"))
            .full_width(self.full_width)
            .with_handle(handle)
            .on_open(std::rc::Rc::new(move |window, cx| {
                menu_for_open.update(cx, |menu, cx| {
                    menu.select_toggled_or_first(window, cx);
                });
            }))
            .menu(move |_window, _cx| Some(self.menu.clone()))
            .trigger(text_button)
            .anchor(Anchor::TopRight)
            .attach(self.attach.unwrap_or(Anchor::BottomRight))
            .when_some(self.offset, |this, offset| this.offset(offset))
    }
}
