use catppuccin::{Flavor, PALETTE};
use fluent::FluentArgs;
use gpui::{Context, FontWeight, SharedString, Subscription, Window, div, prelude::*, px, rgb};
use kbgpui::ui::prelude::*;

use super::{LiveSettings, OpenSettings, open_settings, resolved_theme_mode};
use crate::i18n;
use crate::settings::Settings;
use crate::tui::ThemeMode;

pub(super) struct Welcome {
    settings: Settings,
    session_date: String,
    _settings: Subscription,
    _appearance: Subscription,
}

impl Welcome {
    pub(super) fn new(settings: Settings, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let session_date = i18n::format_session_date(&settings.lang);
        let _settings = cx.observe_global::<LiveSettings>(|this, cx| {
            this.settings = cx.global::<LiveSettings>().0.clone();
            this.session_date = i18n::format_session_date(&this.settings.lang);
            cx.notify();
        });
        let _appearance = cx.observe_window_appearance(window, |_, _, cx| {
            super::apply_theme(cx);
            cx.notify();
        });
        Self {
            settings,
            session_date,
            _settings,
            _appearance,
        }
    }

    fn theme(&self, cx: &gpui::App) -> ThemeMode {
        resolved_theme_mode(self.settings.theme_mode, cx)
    }

    fn flavor(&self, cx: &gpui::App) -> &'static Flavor {
        match self.theme(cx) {
            ThemeMode::Light => &PALETTE.latte,
            ThemeMode::Dark => &PALETTE.mocha,
        }
    }

    fn msg(&self, id: &str) -> SharedString {
        i18n::t_desktop(&self.settings.lang, id, None)
            .into_owned()
            .into()
    }

    fn msg_args(&self, id: &str, args: &FluentArgs<'_>) -> SharedString {
        i18n::t_desktop(&self.settings.lang, id, Some(args))
            .into_owned()
            .into()
    }
}

fn ctp(color: &catppuccin::Color) -> gpui::Rgba {
    let r = u32::from(color.rgb.r);
    let g = u32::from(color.rgb.g);
    let b = u32::from(color.rgb.b);
    rgb((r << 16) | (g << 8) | b)
}

impl gpui::Render for Welcome {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = &self.flavor(cx).colors;
        let base = ctp(&colors.base);
        let crust = ctp(&colors.crust);
        let text = ctp(&colors.text);
        let mauve = ctp(&colors.mauve);
        let subtext1 = ctp(&colors.subtext1);
        let subtext0 = ctp(&colors.subtext0);
        let lavender = ctp(&colors.lavender);
        let surface1 = ctp(&colors.surface1);

        let session_date = self.session_date.clone();
        let showcase = i18n::showcase_args("Alex", &session_date);

        let brand = self.msg("app-brand");
        let title = self.msg("welcome-title");
        let lede = self.msg("welcome-lede");
        let showcase_heading = self.msg("showcase-heading");
        let showcase_intro = self.msg("showcase-intro");
        let placeable = self.msg_args("showcase-placeable", &showcase);
        let plural = self.msg_args("showcase-plural", &showcase);
        let number = self.msg_args("showcase-number", &showcase);
        let date = self.msg_args("showcase-date", &showcase);
        let nested = self.msg("showcase-nested");
        let settings_label = self.msg("settings-open");

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(base)
            .text_color(text)
            .font_family("Figtree")
            .on_action(cx.listener(|_, _: &OpenSettings, _, cx| open_settings(cx)))
            .child(
                div()
                    .absolute()
                    .size_full()
                    .bg(crust)
                    .opacity(0.35)
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .w(px(420.))
                            .h(px(280.))
                            .bg(ctp(&colors.mauve))
                            .opacity(0.18),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .right_0()
                            .w(px(360.))
                            .h(px(240.))
                            .bg(ctp(&colors.blue))
                            .opacity(0.14),
                    )
                    .child(
                        div()
                            .absolute()
                            .bottom_0()
                            .right_0()
                            .w(px(280.))
                            .h(px(280.))
                            .rounded_full()
                            .bg(ctp(&colors.peach))
                            .opacity(0.16),
                    ),
            )
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_col()
                    .size_full()
                    .px_6()
                    .pt_5()
                    .pb_8()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_4()
                            .child(
                                div()
                                    .font_family("Fraunces")
                                    .text_3xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child(brand),
                            )
                            .child(
                                h_flex()
                                    .id("open-settings")
                                    .h(px(28.))
                                    .px_2()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(surface1)
                                    .cursor_pointer()
                                    .child(Label::new(settings_label).size(LabelSize::Small))
                                    .on_click(cx.listener(|_, _, _, cx| open_settings(cx))),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .pt_8()
                            .pb_6()
                            .w(px(560.))
                            .child(
                                div()
                                    .font_family("Fraunces")
                                    .text_3xl()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(mauve)
                                    .child(title),
                            )
                            .child(div().text_lg().text_color(subtext1).w(px(440.)).child(lede)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .w(px(560.))
                            .pt_4()
                            .border_t_1()
                            .border_color(surface1)
                            .child(
                                div()
                                    .font_family("Fraunces")
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(showcase_heading),
                            )
                            .child(div().text_sm().text_color(subtext0).child(showcase_intro))
                            .child(showcase_row(placeable, lavender, subtext1))
                            .child(showcase_row(plural, lavender, subtext1))
                            .child(showcase_row(number, lavender, subtext1))
                            .child(showcase_row(date, lavender, subtext1))
                            .child(showcase_row(nested, lavender, subtext1)),
                    ),
            )
    }
}

fn showcase_row(text: SharedString, accent: gpui::Rgba, color: gpui::Rgba) -> impl IntoElement {
    div()
        .flex()
        .pl_3()
        .border_l_2()
        .border_color(accent)
        .text_sm()
        .text_color(color)
        .child(text)
}
