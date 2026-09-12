//! Application settings resolved through usage-config layers.
//!
//! Precedence follows usage (cli → env → files → defaults). `lang` has no
//! usage `env` binding so `LANG` cannot outrank the persisted UI toggle; system
//! locale is applied only as the declared default.

use std::fs;
use std::path::PathBuf;

use usage::config::{
    resolve, CliLayer, EnvLayer, FileLayer, Layers, SourceKind, XdgBase,
};
use usage::Config;

use crate::i18n::{self, Locale};

const CONFIG_REL: &str = "mediaar/config.toml";

/// How Mediaar behaves, resolved from flags, files, and defaults.
#[derive(Debug, Clone, Config)]
#[usage(file(path = "mediaar/config.toml", xdg = "config", format = "toml"))]
pub struct Settings {
    /// UI language (e.g. en, es)
    #[usage(
        default_fn = default_lang,
        default_note = "from LANG / system, else en",
        cli("--lang")
    )]
    pub lang: String,
}

fn default_lang() -> String {
    i18n::system_locale().unwrap_or_else(|| i18n::FALLBACK.to_string())
}

/// Resolve locale: CLI → user config file (UI toggle) → system default.
pub fn resolve_locale(cli_layer: &CliLayer) -> Locale {
    let env = EnvLayer::from_process();
    let files = FileLayer::xdg(XdgBase::Config, CONFIG_REL);

    let resolved = match resolve(
        Settings::SETTINGS_REGISTRY,
        Layers::new().then(cli_layer).then(&env).then(&files),
    ) {
        Ok(resolved) => resolved,
        Err(err) => {
            eprintln!("config warning: {err}");
            return Locale {
                tag: default_lang(),
                from_cli: false,
            };
        }
    };

    for warning in usage::config::explain::warnings(&resolved) {
        eprintln!("{warning}");
    }

    let settings = match Settings::read(&resolved) {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("config warning: {err}");
            return Locale {
                tag: default_lang(),
                from_cli: false,
            };
        }
    };

    let from_cli = resolved
        .origin_key("lang")
        .is_some_and(|origin| origin.kind == SourceKind::CLI);

    Locale {
        tag: i18n::negotiate(&settings.lang),
        from_cli,
    }
}

/// Persist the UI language toggle into the user XDG config file.
pub fn save_lang(lang: &str) -> Result<(), String> {
    let tag = i18n::negotiate(lang);
    let path = user_config_path().ok_or_else(|| "no config directory".to_owned())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }

    let mut table = fs::read_to_string(&path)
        .ok()
        .and_then(|raw| raw.parse::<toml::Table>().ok())
        .unwrap_or_default();
    table.insert("lang".to_owned(), toml::Value::String(tag));

    let raw = toml::to_string_pretty(&table).map_err(|err| err.to_string())?;
    fs::write(path, raw).map_err(|err| err.to_string())
}

fn user_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join(CONFIG_REL))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use usage::config::CliLayer;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_config_home(lang: &str, f: impl FnOnce()) {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = std::env::temp_dir().join(format!(
            "mediaar-settings-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let cfg = dir.join("mediaar");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(cfg.join("config.toml"), format!("lang = \"{lang}\"\n")).unwrap();

        let previous = std::env::var_os("XDG_CONFIG_HOME");
        // SAFETY: serialized by ENV_LOCK; restored before unlock.
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", &dir);
        }
        f();
        unsafe {
            match previous {
                Some(value) => std::env::set_var("XDG_CONFIG_HOME", value),
                None => std::env::remove_var("XDG_CONFIG_HOME"),
            }
        }
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn file_beats_system_default() {
        with_config_home("es", || {
            let locale = resolve_locale(&CliLayer::new(Vec::<(&str, &str)>::new()));
            assert_eq!(locale.tag, "es");
            assert!(!locale.from_cli);
        });
    }

    #[test]
    fn cli_beats_file() {
        with_config_home("es", || {
            let locale = resolve_locale(&CliLayer::new([("lang", "en")]));
            assert_eq!(locale.tag, "en");
            assert!(locale.from_cli);
        });
    }

    #[test]
    fn save_lang_writes_toml() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = std::env::temp_dir().join(format!(
            "mediaar-settings-write-{}",
            std::process::id()
        ));
        let previous = std::env::var_os("XDG_CONFIG_HOME");
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", &dir);
        }
        save_lang("es").unwrap();
        let raw = fs::read_to_string(dir.join("mediaar/config.toml")).unwrap();
        assert!(raw.contains("lang"), "{raw}");
        assert!(raw.contains("es"), "{raw}");
        unsafe {
            match previous {
                Some(value) => std::env::set_var("XDG_CONFIG_HOME", value),
                None => std::env::remove_var("XDG_CONFIG_HOME"),
            }
        }
        let _ = fs::remove_dir_all(dir);
    }
}
