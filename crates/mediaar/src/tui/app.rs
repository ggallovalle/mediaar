use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use super::theme::{Theme, ThemeMode};

struct App {
    theme: Theme,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            theme: Theme::new(ThemeMode::Dark),
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
            _ => {}
        }
    }
}

pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

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

    let [area] = Layout::vertical([Constraint::Percentage(70)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::horizontal([Constraint::Percentage(70)])
        .flex(Flex::Center)
        .areas(area);

    draw_welcome(frame, area, theme);
}

fn draw_welcome(frame: &mut Frame, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border())
        .style(theme.panel())
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(Span::styled("Mediaar", theme.title())).centered(),
        Line::from(""),
        Line::from(Span::styled("Manage your media", theme.subtitle())).centered(),
        Line::from(""),
        Line::from(vec![
            Span::styled("Theme  ", theme.hint()),
            Span::styled(theme.mode.label(), theme.accent()),
        ])
        .centered(),
        Line::from(""),
        Line::from(Span::styled("t / tab  toggle theme", theme.hint())).centered(),
        Line::from(Span::styled("q / esc  quit", theme.hint())).centered(),
    ];

    let height = u16::try_from(lines.len()).unwrap_or(u16::MAX);
    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    let [content] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(inner);
    frame.render_widget(paragraph, content);
}
