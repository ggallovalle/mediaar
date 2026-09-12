use catppuccin::{Flavor, PALETTE};
use ratatui::style::{Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Light => "Latte",
            Self::Dark => "Mocha",
        }
    }

    pub fn flavor(self) -> &'static Flavor {
        match self {
            Self::Light => &PALETTE.latte,
            Self::Dark => &PALETTE.mocha,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub mode: ThemeMode,
}

impl Theme {
    pub fn new(mode: ThemeMode) -> Self {
        Self { mode }
    }

    pub fn toggle(&mut self) {
        self.mode = self.mode.toggle();
    }

    fn colors(self) -> &'static catppuccin::FlavorColors {
        &self.mode.flavor().colors
    }

    pub fn base(self) -> Style {
        Style::default()
            .fg(self.colors().text.into())
            .bg(self.colors().base.into())
    }

    pub fn title(self) -> Style {
        Style::default()
            .fg(self.colors().mauve.into())
            .bg(self.colors().base.into())
            .add_modifier(Modifier::BOLD)
    }

    pub fn subtitle(self) -> Style {
        Style::default()
            .fg(self.colors().subtext1.into())
            .bg(self.colors().base.into())
    }

    pub fn accent(self) -> Style {
        Style::default()
            .fg(self.colors().blue.into())
            .bg(self.colors().base.into())
            .add_modifier(Modifier::BOLD)
    }

    pub fn hint(self) -> Style {
        Style::default()
            .fg(self.colors().overlay1.into())
            .bg(self.colors().base.into())
    }

    pub fn panel(self) -> Style {
        Style::default()
            .fg(self.colors().text.into())
            .bg(self.colors().mantle.into())
    }

    pub fn border(self) -> Style {
        Style::default()
            .fg(self.colors().surface1.into())
            .bg(self.colors().mantle.into())
    }
}
