use usage::config::{CliLayer, EnvLayer, FileLayer, Layers, XdgBase, resolve};

use super::{CONFIG_REL, Settings};
use crate::i18n;

/// Resolve settings: CLI → env → XDG file → defaults.
pub fn load(cli_layer: &CliLayer) -> Settings {
    let env = EnvLayer::from_process();
    let files = FileLayer::xdg(XdgBase::Config, CONFIG_REL);
    let resolved = match resolve(
        Settings::SETTINGS_REGISTRY,
        Layers::new().then(cli_layer).then(&env).then(&files),
    ) {
        Ok(resolved) => resolved,
        Err(err) => {
            eprintln!("config warning: {err}");
            return fallback();
        }
    };

    for warning in usage::config::explain::warnings(&resolved) {
        eprintln!("{warning}");
    }

    match Settings::read(&resolved) {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("config warning: {err}");
            fallback()
        }
    }
}

fn fallback() -> Settings {
    Settings {
        lang: i18n::system_locale().unwrap_or_else(i18n::fallback).into(),
        theme_mode: super::ThemeAppearanceMode::Dark,
    }
}
