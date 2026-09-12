mod desktop;
mod tui;

use desktop::Desktop;
use tui::Tui;
use usage::{Cli, Subcommands};

/// Manage your media
#[derive(Cli)]
#[usage(bin = "mediaar", name = "Mediaar", version = "0.1.0", completion)]
pub(crate) struct Mediaar {
    #[usage(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
pub(crate) enum Commands {
    Desktop(Desktop),
    Tui(Tui),
}
