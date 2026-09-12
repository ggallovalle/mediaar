//! Format-preserving edits to `config.toml` (`toml_edit` keeps comments and layout).

use std::path::{Path, PathBuf};

use super::CONFIG_REL;

/// User-writable XDG config path (`$XDG_CONFIG_HOME/mediaar/config.toml`).
pub fn user_path() -> PathBuf {
    let rel = Path::new(CONFIG_REL);
    if let Some(home) = std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
        let dir = PathBuf::from(home);
        if dir.is_absolute() {
            return dir.join(rel);
        }
    }
    let mut dir = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    dir.push(".config");
    dir.join(rel)
}

/// Set a string key (`lang` or `theme.mode`) without rewriting unrelated keys or comments.
pub fn set_string(path: &Path, dotted: &str, value: &str) -> Result<(), String> {
    let mut doc = if path.exists() {
        let raw = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        raw.parse::<toml_edit::DocumentMut>()
            .map_err(|err| err.to_string())?
    } else {
        toml_edit::DocumentMut::new()
    };

    match dotted.split('.').collect::<Vec<_>>().as_slice() {
        [key] => doc[*key] = toml_edit::value(value),
        [table, key] => {
            if doc.get(table).is_none() {
                doc[table] = toml_edit::table();
            }
            let item = &mut doc[table];
            if let Some(inline) = item.as_inline_table_mut() {
                inline.insert(key.to_string(), value.into());
            } else {
                item[key] = toml_edit::value(value);
            }
        }
        _ => return Err(format!("unsupported config key `{dotted}`")),
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    std::fs::write(path, doc.to_string()).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_string_keeps_comments_and_unknown_keys() {
        let dir = std::env::temp_dir().join(format!(
            "mediaar-toml-edit-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");
        std::fs::write(&path, "# keep me\nlang = \"en\"\n# after lang\nextra = 1\n").unwrap();

        set_string(&path, "lang", "es").unwrap();
        set_string(&path, "theme.mode", "light").unwrap();

        let raw = std::fs::read_to_string(&path).unwrap();
        assert!(raw.contains("# keep me"), "{raw}");
        assert!(raw.contains("# after lang"), "{raw}");
        assert!(raw.contains("extra = 1"), "{raw}");
        assert!(raw.contains("lang = \"es\""), "{raw}");
        assert!(raw.contains("mode = \"light\""), "{raw}");

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn set_string_keeps_inline_theme_table() {
        let dir = std::env::temp_dir().join(format!(
            "mediaar-toml-inline-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");
        std::fs::write(&path, "lang = \"en\"\ntheme = { mode = \"dark\" }\n").unwrap();

        set_string(&path, "theme.mode", "light").unwrap();

        let raw = std::fs::read_to_string(&path).unwrap();
        assert!(raw.contains("lang = \"en\""), "{raw}");
        assert!(
            raw.contains("theme = {") && raw.contains("mode = \"light\""),
            "{raw}"
        );

        let _ = std::fs::remove_dir_all(dir);
    }
}
