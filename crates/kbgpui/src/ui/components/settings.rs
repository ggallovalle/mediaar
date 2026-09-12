use crate::theme::ActiveTheme;
use crate::ui::prelude::*;
use crate::ui::{Color, Label, LabelCommon, LabelSize};

/// Section title + rule, matching Zed's settings content headers.
#[derive(IntoElement)]
pub struct SettingsSectionHeader {
    label: SharedString,
}

impl SettingsSectionHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl RenderOnce for SettingsSectionHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_1p5()
            .child(
                Label::new(self.label)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(div().h(px(1.)).w_full().bg(cx.theme().colors().border))
    }
}

/// Title + description on the left, control on the right (Zed settings row).
#[derive(IntoElement)]
pub struct SettingsItem {
    title: SharedString,
    description: SharedString,
    control: AnyElement,
}

impl SettingsItem {
    pub fn new(
        title: impl Into<SharedString>,
        description: impl Into<SharedString>,
        control: impl IntoElement,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            control: control.into_any_element(),
        }
    }
}

impl RenderOnce for SettingsItem {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .gap_4()
            .py_3()
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_0p5()
                    .child(Label::new(self.title))
                    .child(
                        Label::new(self.description)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(self.control)
    }
}
