//! Application settings declared for usage-config.
//!
//! Precedence follows usage (cli → env → files → defaults). `lang` has no
//! usage `env` binding so `LANG` cannot outrank the config file; system locale
//! is applied only as the declared default.

mod file;
mod lang;
mod resolve;
mod theme;

use usage::Config;

use crate::i18n;

pub use file::{set_string, user_path};
pub use lang::Lang;
pub use resolve::load;
pub use theme::ThemeAppearanceMode;

pub(crate) const CONFIG_REL: &str = "mediaar/config.toml";

/// How Mediaar behaves, resolved from flags, files, and defaults.
#[derive(Debug, Clone, Config)]
#[usage(file(path = "mediaar/config.toml", xdg = "config", format = "toml"))]
pub struct Settings {
    /// UI language (e.g. en, es)
    #[usage(
        ty = "string",
        default_fn = default_lang,
        default_note = "from LANG / system, else en",
        cli("--lang")
    )]
    pub lang: Lang,

    /// Theme appearance: light, dark, or follow the OS.
    #[usage(
        key = "theme.mode",
        ty = "string",
        default = "dark",
        choices("light", "dark", "system")
    )]
    pub theme_mode: ThemeAppearanceMode,
}

fn default_lang() -> Lang {
    i18n::system_locale().unwrap_or_else(i18n::fallback).into()
}

/// Persist one string key on the user config file.
pub fn write_user(dotted: &str, value: &str) -> Result<(), String> {
    set_string(&user_path(), dotted, value)
}
