use std::{rc::Rc, sync::Arc};

use gpui::Hsla;

use crate::theme::ActiveTheme;
use crate::ui::prelude::*;
use crate::ui::utils::is_light;
use crate::ui::{Color, DynamicSpacing, Label, LabelCommon, LabelSize, h_flex, v_flex};

/// Creates a new switch.
pub fn switch(id: impl Into<ElementId>, toggle_state: ToggleState) -> Switch {
    Switch::new(id, toggle_state)
}

/// Defines the color for a switch component.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum SwitchColor {
    #[default]
    Accent,
    Custom(Hsla),
}

impl SwitchColor {
    fn get_colors(&self, is_on: bool, cx: &App) -> (Hsla, Hsla) {
        if !is_on {
            return (
                cx.theme().colors().element_disabled,
                cx.theme().colors().border,
            );
        }

        match self {
            SwitchColor::Accent => {
                let status = cx.theme().status();
                let colors = cx.theme().colors();
                (status.info.opacity(0.4), colors.text_accent.opacity(0.2))
            }
            SwitchColor::Custom(color) => (*color, color.opacity(0.6)),
        }
    }
}

impl From<SwitchColor> for Color {
    fn from(color: SwitchColor) -> Self {
        match color {
            SwitchColor::Accent => Color::Accent,
            SwitchColor::Custom(_) => Color::Default,
        }
    }
}

/// Defines the color for a switch component.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum SwitchLabelPosition {
    Start,
    #[default]
    End,
}

/// # Switch
///
/// Switches are used to represent opposite states, such as enabled or disabled.
#[derive(IntoElement)]
#[allow(clippy::type_complexity)]
pub struct Switch {
    id: ElementId,
    toggle_state: ToggleState,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>>,
    label: Option<SharedString>,
    label_position: Option<SwitchLabelPosition>,
    label_size: LabelSize,
    label_color: Color,
    full_width: bool,
    color: SwitchColor,
}

impl Switch {
    /// Creates a new [`Switch`].
    pub fn new(id: impl Into<ElementId>, state: ToggleState) -> Self {
        Self {
            id: id.into(),
            toggle_state: state,
            disabled: false,
            on_click: None,
            label: None,
            label_position: None,
            label_size: LabelSize::Small,
            label_color: Color::Default,
            full_width: false,
            color: SwitchColor::default(),
        }
    }

    /// Sets the color of the switch using the specified [`SwitchColor`].
    pub fn color(mut self, color: SwitchColor) -> Self {
        self.color = color;
        self
    }

    /// Sets the disabled state of the [`Switch`].
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Binds a handler to the [`Switch`] that will be called when clicked.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// Sets the label of the [`Switch`].
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn label_position(
        mut self,
        label_position: impl Into<Option<SwitchLabelPosition>>,
    ) -> Self {
        self.label_position = label_position.into();
        self
    }

    pub fn label_size(mut self, size: LabelSize) -> Self {
        self.label_size = size;
        self
    }

    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_on = self.toggle_state == ToggleState::Selected;
        let adjust_ratio = if is_light(cx) { 1.5 } else { 1.0 };

        let base_color = cx.theme().colors().text;
        let thumb_color = base_color;
        let (bg_color, border_color) = self.color.get_colors(is_on, cx);

        let bg_hover_color = if is_on {
            bg_color.blend(base_color.opacity(0.16 * adjust_ratio))
        } else {
            bg_color.blend(base_color.opacity(0.05 * adjust_ratio))
        };

        let thumb_opacity = match (is_on, self.disabled) {
            (_, true) => 0.2,
            (true, false) => 1.0,
            (false, false) => 0.5,
        };

        let group_id = format!("switch_group_{:?}", self.id);
        let label = self.label;

        let switch = div()
            .id((self.id.clone(), "switch"))
            .p(px(1.0))
            .border_2()
            .border_color(cx.theme().colors().border_transparent)
            .rounded_full()
            .child(
                h_flex()
                    .w(DynamicSpacing::Base32.rems(cx))
                    .h(DynamicSpacing::Base20.rems(cx))
                    .group(group_id.clone())
                    .child(
                        h_flex()
                            .when(is_on, |on| on.justify_end())
                            .when(!is_on, |off| off.justify_start())
                            .size_full()
                            .rounded_full()
                            .px(DynamicSpacing::Base02.px(cx))
                            .bg(bg_color)
                            .when(!self.disabled, |this| {
                                this.group_hover(group_id.clone(), |el| el.bg(bg_hover_color))
                            })
                            .border_1()
                            .border_color(border_color)
                            .child(
                                div()
                                    .size(DynamicSpacing::Base12.rems(cx))
                                    .rounded_full()
                                    .bg(thumb_color)
                                    .opacity(thumb_opacity),
                            ),
                    ),
            );

        h_flex()
            .id(self.id)
            .cursor_pointer()
            .gap(DynamicSpacing::Base06.rems(cx))
            .when(self.full_width, |this| this.w_full().justify_between())
            .when(
                self.label_position == Some(SwitchLabelPosition::Start),
                |this| {
                    this.when_some(label.clone(), |this, label| {
                        this.child(
                            Label::new(label)
                                .size(self.label_size)
                                .color(self.label_color),
                        )
                    })
                },
            )
            .child(switch)
            .when(
                self.label_position == Some(SwitchLabelPosition::End),
                |this| {
                    this.when_some(label, |this, label| {
                        this.child(
                            Label::new(label)
                                .size(self.label_size)
                                .color(self.label_color),
                        )
                    })
                },
            )
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        cx.stop_propagation();
                        on_click(&self.toggle_state.inverse(), window, cx)
                    })
                },
            )
    }
}

/// # SwitchField
///
/// A field component that combines a label, description, and switch into one reusable component.
///
/// # Examples
///
/// ```
/// use kbgpui::ui::prelude::*;
/// use kbgpui::ui::{SwitchField, ToggleState};
///
/// let switch_field = SwitchField::new(
///     "feature-toggle",
///     Some("Enable feature"),
///     Some("This feature adds new functionality to the app.".into()),
///     ToggleState::Unselected,
///     |_, _, _| {
///         // Logic here
///     }
/// );
/// ```
#[derive(IntoElement)]
#[allow(clippy::type_complexity)]
pub struct SwitchField {
    id: ElementId,
    label: Option<SharedString>,
    description: Option<SharedString>,
    toggle_state: ToggleState,
    on_click: Arc<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>,
    disabled: bool,
    color: SwitchColor,
}

impl SwitchField {
    pub fn new(
        id: impl Into<ElementId>,
        label: Option<impl Into<SharedString>>,
        description: Option<SharedString>,
        toggle_state: impl Into<ToggleState>,
        on_click: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.map(Into::into),
            description,
            toggle_state: toggle_state.into(),
            on_click: Arc::new(on_click),
            disabled: false,
            color: SwitchColor::Accent,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the color of the switch using the specified [`SwitchColor`].
    /// This changes the color scheme of the switch when it's in the "on" state.
    pub fn color(mut self, color: SwitchColor) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for SwitchField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .id((self.id.clone(), "container"))
            .when(!self.disabled, |this| {
                this.hover(|this| this.cursor_pointer())
            })
            .w_full()
            .gap_4()
            .justify_between()
            .flex_wrap()
            .child(match &self.description {
                Some(description) => v_flex()
                    .gap_0p5()
                    .max_w_5_6()
                    .when_some(self.label, |this, label| this.child(Label::new(label)))
                    .child(Label::new(description.clone()).color(Color::Muted))
                    .into_any_element(),
                None => {
                    if let Some(label) = self.label.clone() {
                        Label::new(label).into_any_element()
                    } else {
                        gpui::Empty.into_any_element()
                    }
                }
            })
            .child(
                Switch::new((self.id.clone(), "switch"), self.toggle_state)
                    .color(self.color)
                    .disabled(self.disabled)
                    .on_click({
                        let on_click = self.on_click.clone();
                        move |state, window, cx| {
                            (on_click)(state, window, cx);
                        }
                    }),
            )
            .when(!self.disabled, |this| {
                this.on_click({
                    let on_click = self.on_click.clone();
                    let toggle_state = self.toggle_state;
                    move |_click, window, cx| {
                        (on_click)(&toggle_state.inverse(), window, cx);
                    }
                })
            })
    }
}
