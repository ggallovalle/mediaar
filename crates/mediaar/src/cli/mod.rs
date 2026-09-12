mod desktop;
mod tui;

use desktop::Desktop;
use tui::Tui;
use usage::{Cli, Subcommands};

use crate::settings::Settings;

/// Manage your media
#[derive(Cli)]
#[usage(
    bin = "mediaar",
    name = "Mediaar",
    version = "0.1.0",
    completion,
    config = Settings
)]
pub(crate) struct Mediaar {
    /// UI language (e.g. en, es). Overrides the persisted toggle and system locale.
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
