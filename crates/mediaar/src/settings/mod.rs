//! Application settings declared for usage-config.
//!
//! Precedence follows usage (cli → env → files → defaults). `lang` has no
//! usage `env` binding so `LANG` cannot outrank the config file; system locale
//! is applied only as the declared default.

mod lang;
mod resolve;

use usage::Config;

use crate::i18n;

pub use lang::Lang;
pub use resolve::load;

pub(crate) const CONFIG_REL: &str = "mediaar/config.toml";

/// How Mediaar behaves, resolved from flags, files, and defaults.
#[derive(Debug, Clone, Config)]
#[usage(file(path = "mediaar/config.toml", xdg = "config", format = "toml"))]
pub struct Settings {
    /// UI language ([`unic_langid::LanguageIdentifier`])
    #[usage(
        ty = "string",
        default_fn = default_lang,
        default_note = "from LANG / system, else en",
        cli("--lang")
    )]
    pub lang: Lang,
}

fn default_lang() -> Lang {
    i18n::system_locale().unwrap_or_else(i18n::fallback).into()
}
