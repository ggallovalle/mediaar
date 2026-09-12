//! In-process GPUI desktop shell embedded in the `mediaar` binary.

mod settings_window;
mod welcome;

use gpui::{
    App, Application, Bounds, Global, KeyBinding, TitlebarOptions, WindowAppearance, WindowBounds,
    WindowHandle, WindowOptions, actions, prelude::*, px, size,
};
use kbgpui::theme::{self, Theme};

use crate::cli::CliState;
use crate::settings::{Settings, ThemeAppearanceMode};
use crate::tui::ThemeMode;

use settings_window::SettingsWindow;
use welcome::Welcome;

actions!(mediaar, [OpenSettings]);

struct LiveSettings(Settings);

impl Global for LiveSettings {}

struct SettingsUiHandle(Option<WindowHandle<SettingsWindow>>);

impl Global for SettingsUiHandle {}

fn write_bench_ready() {
    if let Ok(path) = std::env::var("MEDIAAR_BENCH_READY_FILE") {
        let _ = std::fs::write(path, b"ready\n");
    }
}

fn is_os_light(cx: &App) -> bool {
    matches!(
        cx.window_appearance(),
        WindowAppearance::Light | WindowAppearance::VibrantLight
    )
}

fn resolved_theme_mode(mode: ThemeAppearanceMode, cx: &App) -> ThemeMode {
    match mode {
        ThemeAppearanceMode::Light => ThemeMode::Light,
        ThemeAppearanceMode::Dark => ThemeMode::Dark,
        ThemeAppearanceMode::System => {
            if is_os_light(cx) {
                ThemeMode::Light
            } else {
                ThemeMode::Dark
            }
        }
    }
}

fn apply_theme(cx: &mut App) {
    let mode = cx.global::<LiveSettings>().0.theme_mode;
    theme::set_theme(
        cx,
        match resolved_theme_mode(mode, cx) {
            ThemeMode::Light => Theme::latte(),
            ThemeMode::Dark => Theme::mocha(),
        },
    );
}

fn persist(cx: &mut App, dotted: &str, value: &str) {
    if let Err(err) = crate::settings::write_user(dotted, value) {
        eprintln!("config warning: {err}");
    }
    cx.update_global::<LiveSettings, _>(|live, _cx| match dotted {
        "lang" => {
            if let Ok(lang) = value.parse() {
                live.0.lang = lang;
            }
        }
        "theme.mode" => {
            if let Ok(mode) = value.parse() {
                live.0.theme_mode = mode;
            }
        }
        _ => {}
    });
    apply_theme(cx);
}

fn open_settings(cx: &mut App) {
    if let Some(handle) = cx.global::<SettingsUiHandle>().0
        && handle
            .update(cx, |_, window, _| {
                window.activate_window();
            })
            .is_ok()
    {
        return;
    }

    let bounds = Bounds::centered(None, size(px(860.), px(560.)), cx);
    let handle = cx
        .open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Mediaar — Settings".into()),
                    ..Default::default()
                }),
                focus: true,
                show: true,
                ..Default::default()
            },
            |window, cx| cx.new(|cx| SettingsWindow::new(window, cx)),
        )
        .expect("open settings window");

    cx.global_mut::<SettingsUiHandle>().0 = Some(handle);
}

pub fn run(state: CliState) {
    Application::new()
        .with_assets(kbgpui::Assets)
        .run(move |cx: &mut App| {
            theme::init(cx);
            kbgpui::ui::init(cx);
            cx.set_global(LiveSettings(state.settings.clone()));
            cx.set_global(SettingsUiHandle(None));
            apply_theme(cx);

            cx.on_action(|_: &OpenSettings, cx| open_settings(cx));
            cx.bind_keys([KeyBinding::new("ctrl-,", OpenSettings, None)]);

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
                move |window, cx| {
                    write_bench_ready();
                    cx.new(|cx| Welcome::new(state.settings.clone(), window, cx))
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
