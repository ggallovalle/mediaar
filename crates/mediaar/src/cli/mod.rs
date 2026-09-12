mod desktop;
mod tui;

use desktop::Desktop;
use tui::Tui;
use usage::{Cli, Subcommands};

use crate::settings::Settings;

/// Shared state handed to every subcommand after settings resolve.
#[derive(Debug, Clone)]
pub(crate) struct CliState {
    pub(crate) settings: Settings,
}

/// Manage your media
#[derive(Cli)]
#[usage(
    bin = "mediaar",
    name = "Mediaar",
    version = "0.2.0",
    completion,
    config = Settings
)]
pub(crate) struct Mediaar {
    /// UI language (e.g. en, es). Overrides the config file and system locale.
    #[usage(long = "lang", global, setting = "lang")]
    pub(crate) lang: Option<String>,

    #[usage(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommands)]
#[usage(run_with)]
pub(crate) enum Commands {
    Desktop(Desktop),
    Tui(Tui),
}
