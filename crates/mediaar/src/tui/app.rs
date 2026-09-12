use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use fluent::FluentArgs;
use fluent::FluentValue;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use crate::i18n::{self, Locale};

use super::theme::{Theme, ThemeMode};

struct App {
    theme: Theme,
    locale: Locale,
    should_quit: bool,
}

impl App {
    fn new(locale: Locale) -> Self {
        Self {
            theme: Theme::new(ThemeMode::Dark),
            locale,
            should_quit: false,
        }
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        match (code, modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL)
            | (KeyCode::Char('q'), _)
            | (KeyCode::Esc, _) => {
                self.should_quit = true;
            }
            (KeyCode::Char('t'), _) | (KeyCode::Tab, _) => {
                self.theme.toggle();
            }
            (KeyCode::Char('l'), _) => {
                let next = i18n::cycle(self.locale.as_str());
                if let Err(err) = crate::settings::save_lang(&next) {
                    eprintln!("failed to persist language: {err}");
                }
                self.locale.tag = next;
                self.locale.from_cli = false;
            }
            _ => {}
        }
    }
}

pub fn run(locale: Locale) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal, locale);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal, locale: Locale) -> io::Result<()> {
    let mut app = App::new(locale);


    loop {
        terminal.draw(|frame| draw(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    app.handle_key(key.code, key.modifiers);
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn draw(frame: &mut Frame, app: &App) {
    let theme = app.theme;
    frame.render_widget(Clear, frame.area());
    frame.render_widget(Block::default().style(theme.base()), frame.area());

    let [area] = Layout::vertical([Constraint::Percentage(80)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::horizontal([Constraint::Percentage(78)])
        .flex(Flex::Center)
        .areas(area);

    draw_welcome(frame, area, app);
}

fn draw_welcome(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let lang = app.locale.as_str();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border())
        .style(theme.panel())
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let theme_key = match theme.mode {
        ThemeMode::Light => "latte",
        ThemeMode::Dark => "mocha",
    };

    let mut theme_args = FluentArgs::new();
    theme_args.set("theme", FluentValue::from(theme_key));

    let mut lang_args = FluentArgs::new();
    lang_args.set("lang", FluentValue::from(lang));

    let session_date = i18n::format_session_date(lang);
    let showcase = i18n::showcase_args("Alex", &session_date);

    let brand = i18n::t(lang, "app-brand", None);
    let tagline = i18n::t(lang, "tui-tagline", None);
    let theme_label = i18n::t(lang, "tui-theme-label", None);
    let theme_value = i18n::t_attr(lang, "theme-toggle", "label", Some(&theme_args));
    let lang_label = i18n::t(lang, "tui-lang-label", None);
    let lang_value = i18n::t_attr(lang, "lang-toggle", "label", Some(&lang_args));
    let placeable = i18n::t(lang, "showcase-placeable", Some(&showcase));
    let plural = i18n::t(lang, "showcase-plural", Some(&showcase));
    let number = i18n::t(lang, "showcase-number", Some(&showcase));
    let date = i18n::t(lang, "showcase-date", Some(&showcase));
    let nested = i18n::t(lang, "showcase-nested", Some(&showcase));
    let help_theme = i18n::t(lang, "tui-help-theme", None);
    let help_lang = i18n::t(lang, "tui-help-lang", None);
    let help_quit = i18n::t(lang, "tui-help-quit", None);

    let lines = vec![
        Line::from(Span::styled(brand.into_owned(), theme.title())).centered(),
        Line::from(""),
        Line::from(Span::styled(tagline.into_owned(), theme.subtitle())).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled(format!("{theme_label}  "), theme.hint()),
            Span::styled(theme_value.into_owned(), theme.accent()),
        ])
        .centered(),
        Line::from(vec![
            Span::styled(format!("{lang_label}  "), theme.hint()),
            Span::styled(lang_value.into_owned(), theme.accent()),
        ])
        .centered(),
        Line::from(""),
        Line::from(Span::styled(placeable.into_owned(), theme.subtitle())).centered(),
        Line::from(Span::styled(plural.into_owned(), theme.subtitle())).centered(),
        Line::from(Span::styled(number.into_owned(), theme.subtitle())).centered(),
        Line::from(Span::styled(date.into_owned(), theme.subtitle())).centered(),
        Line::from(Span::styled(nested.into_owned(), theme.hint())).centered(),
        Line::from(""),
        Line::from(Span::styled(help_theme.into_owned(), theme.hint())).centered(),
        Line::from(Span::styled(help_lang.into_owned(), theme.hint())).centered(),
        Line::from(Span::styled(help_quit.into_owned(), theme.hint())).centered(),
    ];

    let height = u16::try_from(lines.len()).unwrap_or(u16::MAX);
    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    let [content] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(inner);
    frame.render_widget(paragraph, content);
}
