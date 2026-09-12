use gpui::{
    Context, Entity, Focusable, SharedString, Subscription, Window, div, point, prelude::*, px,
};
use kbgpui::theme::ActiveTheme;
use kbgpui::ui::prelude::*;
use kbgpui::ui::{Input, InputEvent};

use super::{LiveSettings, OpenSettings, persist};
use crate::i18n;
use crate::settings::{Settings, ThemeAppearanceMode};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    General,
    Appearance,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NavTarget {
    General,
    Language,
    Appearance,
    Theme,
}

pub(super) struct SettingsWindow {
    page: Page,
    search_bar: Entity<Input>,
    general_expanded: bool,
    appearance_expanded: bool,
    settings: Settings,
    _search: Subscription,
    _settings: Subscription,
    _appearance: Subscription,
}

impl SettingsWindow {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = cx.global::<LiveSettings>().0.clone();
        let search_bar = cx.new(|cx| {
            let mut editor = Input::single_line(window, cx);
            editor.set_placeholder_text(
                i18n::t_desktop(&settings.lang, "settings-search", None).into_owned(),
                cx,
            );
            editor
        });
        search_bar.focus_handle(cx).focus(window);
        let _search = cx.subscribe(&search_bar, |this, _, _: &InputEvent, cx| {
            this.sync_page_to_query(cx);
            cx.notify();
        });
        let _settings = cx.observe_global::<LiveSettings>(|this, cx| {
            this.settings = cx.global::<LiveSettings>().0.clone();
            let placeholder =
                i18n::t_desktop(&this.settings.lang, "settings-search", None).into_owned();
            this.search_bar.update(cx, |editor, cx| {
                editor.set_placeholder_text(placeholder, cx);
            });
            cx.notify();
        });
        let _appearance = cx.observe_window_appearance(window, |_, _, cx| {
            super::apply_theme(cx);
            cx.notify();
        });
        Self {
            page: Page::Appearance,
            search_bar,
            general_expanded: true,
            appearance_expanded: true,
            settings,
            _search,
            _settings,
            _appearance,
        }
    }

    fn msg(&self, id: &str) -> SharedString {
        i18n::t_desktop(&self.settings.lang, id, None)
            .into_owned()
            .into()
    }

    fn query(&self, cx: &gpui::App) -> String {
        self.search_bar.read(cx).text()
    }

    fn matches(&self, parts: &[&str], cx: &gpui::App) -> bool {
        let query = self.query(cx);
        let query = query.trim();
        if query.is_empty() {
            return true;
        }
        let q = query.to_lowercase();
        parts.iter().any(|part| part.to_lowercase().contains(&q))
    }

    fn show_language(&self, cx: &gpui::App) -> bool {
        self.matches(
            &[
                self.msg("settings-general").as_ref(),
                self.msg("settings-language").as_ref(),
                self.msg("settings-language-desc").as_ref(),
            ],
            cx,
        )
    }

    fn show_theme(&self, cx: &gpui::App) -> bool {
        self.matches(
            &[
                self.msg("settings-appearance").as_ref(),
                self.msg("settings-theme").as_ref(),
                self.msg("settings-theme-mode").as_ref(),
                self.msg("settings-theme-mode-desc").as_ref(),
                self.msg("settings-mode-light").as_ref(),
                self.msg("settings-mode-dark").as_ref(),
                self.msg("settings-mode-system").as_ref(),
            ],
            cx,
        )
    }

    fn open_nav(&mut self, target: NavTarget, cx: &mut Context<Self>) {
        match target {
            NavTarget::General => {
                self.page = Page::General;
                self.general_expanded = true;
            }
            NavTarget::Language => {
                self.page = Page::General;
                self.general_expanded = true;
            }
            NavTarget::Appearance => {
                self.page = Page::Appearance;
                self.appearance_expanded = true;
            }
            NavTarget::Theme => {
                self.page = Page::Appearance;
                self.appearance_expanded = true;
            }
        }
        cx.notify();
    }

    fn sync_page_to_query(&mut self, cx: &gpui::App) {
        let show_language = self.show_language(cx);
        let show_theme = self.show_theme(cx);
        match self.page {
            Page::General if !show_language && show_theme => self.page = Page::Appearance,
            Page::Appearance if !show_theme && show_language => self.page = Page::General,
            _ => {}
        }
    }
}

impl gpui::Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let background = cx.theme().colors().background;
        let text = cx.theme().colors().text;
        let border = cx.theme().colors().border;
        let page = self.page;
        let show_language = self.show_language(cx);
        let show_theme = self.show_theme(cx);

        h_flex()
            .size_full()
            .bg(background)
            .text_color(text)
            .font_family("Figtree")
            .on_action(cx.listener(|_, _: &OpenSettings, _, _| {}))
            .child(self.render_nav(window, cx))
            .child(div().w(px(1.)).h_full().bg(border))
            .child(
                v_flex()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .px_8()
                    .py_6()
                    .gap_2()
                    .child(
                        Label::new(self.msg(match page {
                            Page::General => "settings-general",
                            Page::Appearance => "settings-appearance",
                        }))
                        .size(LabelSize::Large),
                    )
                    .when(page == Page::General && show_language, |this| {
                        this.child(self.render_general(window, cx))
                    })
                    .when(page == Page::Appearance && show_theme, |this| {
                        this.child(self.render_appearance(window, cx))
                    }),
            )
    }
}

impl SettingsWindow {
    fn render_search(&self, _window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id("settings-ui-search")
            .track_focus(&self.search_bar.focus_handle(cx))
            .py_1()
            .px_1p5()
            .mb_3()
            .gap_1p5()
            .rounded_sm()
            .bg(cx.theme().colors().editor_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .cursor_text()
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.search_bar.focus_handle(cx).focus(window);
                }),
            )
            .child(Icon::new(IconName::MagnifyingGlass).color(Color::Muted))
            .child(self.search_bar.clone())
    }

    fn render_nav(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panel = cx.theme().colors().panel_background;
        let searching = !self.search_bar.read(cx).is_empty();
        let show_language = self.show_language(cx);
        let show_theme = self.show_theme(cx);
        let show_general =
            show_language || self.matches(&[self.msg("settings-general").as_ref()], cx);
        let show_appearance =
            show_theme || self.matches(&[self.msg("settings-appearance").as_ref()], cx);
        let general_expanded = self.general_expanded || searching;
        let appearance_expanded = self.appearance_expanded || searching;

        v_flex()
            .w(px(226.))
            .h_full()
            .p_2p5()
            .flex_none()
            .bg(panel)
            .child(self.render_search(window, cx))
            .child(
                v_flex()
                    .id("settings-ui-nav")
                    .flex_1()
                    .gap_0()
                    .when(show_general, |this| {
                        this.child(
                            TreeViewItem::new("nav-general", self.msg("settings-general"))
                                .root_item(true)
                                .expanded(general_expanded)
                                .on_toggle(cx.listener(|this, _, _, cx| {
                                    this.general_expanded = !this.general_expanded;
                                    cx.notify();
                                }))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.open_nav(NavTarget::General, cx);
                                })),
                        )
                        .when(general_expanded && show_language, |this| {
                            this.child(
                                TreeViewItem::new("nav-language", self.msg("settings-language"))
                                    .toggle_state(self.page == Page::General)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.open_nav(NavTarget::Language, cx);
                                    })),
                            )
                        })
                    })
                    .when(show_appearance, |this| {
                        this.child(
                            TreeViewItem::new("nav-appearance", self.msg("settings-appearance"))
                                .root_item(true)
                                .expanded(appearance_expanded)
                                .on_toggle(cx.listener(|this, _, _, cx| {
                                    this.appearance_expanded = !this.appearance_expanded;
                                    cx.notify();
                                }))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.open_nav(NavTarget::Appearance, cx);
                                })),
                        )
                        .when(appearance_expanded && show_theme, |this| {
                            this.child(
                                TreeViewItem::new("nav-theme", self.msg("settings-theme"))
                                    .toggle_state(self.page == Page::Appearance)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.open_nav(NavTarget::Theme, cx);
                                    })),
                            )
                        })
                    }),
            )
    }

    fn render_general(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tags = i18n::shipped();
        let current = i18n::catalog_key(&self.settings.lang);
        let selected = tags.iter().position(|tag| *tag == current).unwrap_or(0);
        let labels: Vec<SharedString> = tags
            .iter()
            .map(|tag| {
                let mut args = fluent::FluentArgs::new();
                args.set("lang", fluent::FluentValue::from(*tag));
                i18n::t_attr_desktop(&self.settings.lang, "lang-toggle", "label", Some(&args))
                    .into_owned()
                    .into()
            })
            .collect();
        let current_label = labels.get(selected).cloned().unwrap_or_default();
        let menu_labels = labels.clone();
        let menu = window.use_keyed_state(("lang-menu", selected), cx, move |window, cx| {
            ContextMenu::new(window, cx, move |mut menu, _, _| {
                for (index, label) in menu_labels.iter().enumerate() {
                    let toggled = index == selected;
                    menu = menu.toggleable_entry(
                        label.clone(),
                        toggled,
                        IconPosition::End,
                        None,
                        move |_, cx| {
                            persist(cx, "lang", tags[index]);
                        },
                    );
                }
                menu
            })
        });

        v_flex()
            .w_full()
            .gap_2()
            .child(SettingsSectionHeader::new(self.msg("settings-language")))
            .child(SettingsItem::new(
                self.msg("settings-language"),
                self.msg("settings-language-desc"),
                DropdownMenu::new("lang-dropdown", current_label, menu)
                    .style(DropdownStyle::Outlined)
                    .trigger_size(ButtonSize::Medium)
                    .offset(point(px(0.0_f32), px(2.0_f32))),
            ))
    }

    fn render_appearance(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let modes = ThemeAppearanceMode::ALL;
        let selected = modes
            .iter()
            .position(|mode| *mode == self.settings.theme_mode)
            .unwrap_or(1);
        let labels: Vec<SharedString> = modes.iter().map(|mode| self.mode_label(*mode)).collect();
        let current_label = labels.get(selected).cloned().unwrap_or_default();
        let menu_labels = labels.clone();
        let menu = window.use_keyed_state(("theme-mode-menu", selected), cx, move |window, cx| {
            ContextMenu::new(window, cx, move |mut menu, _, _| {
                for (index, label) in menu_labels.iter().enumerate() {
                    let toggled = index == selected;
                    menu = menu.toggleable_entry(
                        label.clone(),
                        toggled,
                        IconPosition::End,
                        None,
                        move |_, cx| {
                            persist(cx, "theme.mode", modes[index].as_str());
                        },
                    );
                }
                menu
            })
        });

        v_flex()
            .w_full()
            .gap_2()
            .child(SettingsSectionHeader::new(self.msg("settings-theme")))
            .child(SettingsItem::new(
                self.msg("settings-theme-mode"),
                self.msg("settings-theme-mode-desc"),
                DropdownMenu::new("theme-mode-dropdown", current_label, menu)
                    .style(DropdownStyle::Outlined)
                    .trigger_size(ButtonSize::Medium)
                    .offset(point(px(0.0_f32), px(2.0_f32))),
            ))
    }

    fn mode_label(&self, mode: ThemeAppearanceMode) -> SharedString {
        let id = match mode {
            ThemeAppearanceMode::Light => "settings-mode-light",
            ThemeAppearanceMode::Dark => "settings-mode-dark",
            ThemeAppearanceMode::System => "settings-mode-system",
        };
        self.msg(id)
    }
}
