//! In-process GPUI desktop shell embedded in the `mediaar` binary.
//!
//! Experiment branch: replaces the former Tauri + Solid welcome UI.

use catppuccin::{Flavor, PALETTE};
use fluent::{FluentArgs, FluentValue};
use gpui::{
    App, Application, Bounds, ClickEvent, Context, FontWeight, SharedString, TitlebarOptions,
    Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};

use crate::i18n::{self, Locale};
use crate::settings;
use crate::tui::ThemeMode;

fn write_bench_ready() {
    if let Ok(path) = std::env::var("MEDIAAR_BENCH_READY_FILE") {
        let _ = std::fs::write(path, b"ready\n");
    }
}

fn ctp(color: &catppuccin::Color) -> gpui::Rgba {
    let r = u32::from(color.rgb.r);
    let g = u32::from(color.rgb.g);
    let b = u32::from(color.rgb.b);
    rgb((r << 16) | (g << 8) | b)
}

struct Welcome {
    locale: String,
    theme: ThemeMode,
    session_date: String,
}

impl Welcome {
    fn new(locale: Locale) -> Self {
        let tag = locale.tag;
        let session_date = i18n::format_session_date(&tag);
        Self {
            locale: tag,
            theme: ThemeMode::Dark,
            session_date,
        }
    }

    fn flavor(&self) -> &'static Flavor {
        match self.theme {
            ThemeMode::Light => &PALETTE.latte,
            ThemeMode::Dark => &PALETTE.mocha,
        }
    }

    fn theme_key(&self) -> &'static str {
        match self.theme {
            ThemeMode::Light => "latte",
            ThemeMode::Dark => "mocha",
        }
    }

    fn next_theme_key(&self) -> &'static str {
        match self.theme {
            ThemeMode::Light => "mocha",
            ThemeMode::Dark => "latte",
        }
    }

    fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        self.theme = self.theme.toggle();
        cx.notify();
    }

    fn toggle_lang(&mut self, cx: &mut Context<Self>) {
        let next = i18n::cycle(&self.locale);
        if let Err(err) = settings::save_lang(&next) {
            eprintln!("failed to persist language: {err}");
        }
        self.locale = next;
        self.session_date = i18n::format_session_date(&self.locale);
        cx.notify();
    }

    fn msg(&self, id: &str) -> SharedString {
        i18n::t_desktop(&self.locale, id, None).into_owned().into()
    }

    fn msg_args(&self, id: &str, args: &FluentArgs<'_>) -> SharedString {
        i18n::t_desktop(&self.locale, id, Some(args))
            .into_owned()
            .into()
    }

    fn attr(&self, id: &str, attr: &str, args: Option<&FluentArgs<'_>>) -> SharedString {
        i18n::t_attr_desktop(&self.locale, id, attr, args)
            .into_owned()
            .into()
    }
}

impl Render for Welcome {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = &self.flavor().colors;
        let base = ctp(&colors.base);
        let crust = ctp(&colors.crust);
        let text = ctp(&colors.text);
        let mauve = ctp(&colors.mauve);
        let subtext1 = ctp(&colors.subtext1);
        let subtext0 = ctp(&colors.subtext0);
        let lavender = ctp(&colors.lavender);
        let surface1 = ctp(&colors.surface1);
        let mantle = ctp(&colors.mantle);
        let surface0 = ctp(&colors.surface0);

        let theme_key = self.theme_key().to_owned();
        let next_theme_key = self.next_theme_key().to_owned();
        let locale_tag = self.locale.clone();
        let session_date = self.session_date.clone();

        let mut theme_args = FluentArgs::new();
        theme_args.set("theme", FluentValue::from(theme_key.as_str()));
        theme_args.set("next", FluentValue::from(next_theme_key.as_str()));

        let mut lang_args = FluentArgs::new();
        lang_args.set("lang", FluentValue::from(locale_tag.as_str()));

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

        let lang_label = self.attr("lang-toggle", "label", Some(&lang_args));
        let lang_hint = self.attr("lang-toggle", "hint", None);
        let theme_label = self.attr("theme-toggle", "label", Some(&theme_args));
        let theme_hint = self.attr("theme-toggle", "hint", Some(&theme_args));

        let toggle_colors = ToggleColors {
            border: surface1,
            bg: mantle,
            text,
            hint: subtext0,
            hover_border: lavender,
            hover_bg: surface0,
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(base)
            .text_color(text)
            .font_family("Figtree")
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
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(toggle_button(
                                        "lang-toggle",
                                        lang_label,
                                        lang_hint,
                                        toggle_colors,
                                        cx.listener(|this, _, _, cx| this.toggle_lang(cx)),
                                    ))
                                    .child(toggle_button(
                                        "theme-toggle",
                                        theme_label,
                                        theme_hint,
                                        toggle_colors,
                                        cx.listener(|this, _, _, cx| this.toggle_theme(cx)),
                                    )),
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

#[derive(Clone, Copy)]
struct ToggleColors {
    border: gpui::Rgba,
    bg: gpui::Rgba,
    text: gpui::Rgba,
    hint: gpui::Rgba,
    hover_border: gpui::Rgba,
    hover_bg: gpui::Rgba,
}

fn toggle_button(
    id: &'static str,
    label: SharedString,
    hint: SharedString,
    colors: ToggleColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_col()
        .items_end()
        .gap_1()
        .px_3()
        .py_2()
        .rounded_lg()
        .border_1()
        .border_color(colors.border)
        .bg(colors.bg)
        .text_color(colors.text)
        .cursor_pointer()
        .hover(move |style| style.border_color(colors.hover_border).bg(colors.hover_bg))
        .on_click(on_click)
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .child(label),
        )
        .child(div().text_xs().text_color(colors.hint).child(hint))
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

pub fn run(locale: Locale) {
    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(960.), px(640.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Mediaar".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            move |_, cx| {
                write_bench_ready();
                cx.new(|_| Welcome::new(locale))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
