mod cli;
mod desktop;
mod i18n;
mod settings;
mod tui;

use cli::{CliState, Mediaar};
use settings::Settings;
use usage::RunWith;

fn main() {
    let (cli, cli_layer) = Mediaar::parse_with_settings();
    debug_assert_eq!(
        Settings::SETTINGS_REGISTRY.drift(Mediaar::SETTINGS_BINDINGS),
        Vec::<String>::new()
    );
    cli.command.run_with(CliState {
        settings: settings::load(&cli_layer),
    });
}
