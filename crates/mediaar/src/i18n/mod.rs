//! Shared Fluent localization for TUI and desktop.

use std::borrow::Cow;
use std::sync::OnceLock;

use fluent::concurrent::FluentBundle;
use fluent::{FluentArgs, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

/// Shared + surface resources embedded in the binary.
const EN_COMMON: &str = include_str!("../../locales/en/common.ftl");
const EN_DESKTOP: &str = include_str!("../../locales/en/desktop.ftl");
const EN_TUI: &str = include_str!("../../locales/en/tui.ftl");
const ES_COMMON: &str = include_str!("../../locales/es/common.ftl");
const ES_DESKTOP: &str = include_str!("../../locales/es/desktop.ftl");
const ES_TUI: &str = include_str!("../../locales/es/tui.ftl");

/// Locales shipped with the binary / UI bundle.
pub const AVAILABLE: &[&str] = &["en", "es"];

pub const FALLBACK: &str = "en";

type Bundle = FluentBundle<FluentResource>;

static EN_TUI_BUNDLE: OnceLock<Bundle> = OnceLock::new();
static ES_TUI_BUNDLE: OnceLock<Bundle> = OnceLock::new();
static EN_DESKTOP_BUNDLE: OnceLock<Bundle> = OnceLock::new();
static ES_DESKTOP_BUNDLE: OnceLock<Bundle> = OnceLock::new();

/// Resolved UI language for a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locale {
    /// Canonical tag among [`AVAILABLE`] (e.g. `en`, `es`).
    pub tag: String,
    /// True when `--lang` selected this locale for the current process.
    pub from_cli: bool,
}

impl Locale {
    pub fn as_str(&self) -> &str {
        &self.tag
    }
}

/// Map a user/system tag onto a shipped locale.
pub fn negotiate(raw: &str) -> String {
    let normalized = normalize_tag(raw);
    if AVAILABLE.contains(&normalized.as_str()) {
        return normalized;
    }

    let primary = normalized.split('-').next().unwrap_or(FALLBACK);
    if AVAILABLE.contains(&primary) {
        return primary.to_string();
    }

    FALLBACK.to_string()
}

/// Next locale in the cycle (for UI / TUI toggles).
pub fn cycle(current: &str) -> String {
    let current = negotiate(current);
    let idx = AVAILABLE
        .iter()
        .position(|tag| *tag == current)
        .unwrap_or(0);
    let next = AVAILABLE[(idx + 1) % AVAILABLE.len()];
    next.to_string()
}

fn normalize_tag(raw: &str) -> String {
    let trimmed = raw.trim().replace('_', "-");
    let without_encoding = trimmed.split('.').next().unwrap_or(&trimmed);
    let primary = without_encoding
        .split('@')
        .next()
        .unwrap_or(without_encoding);
    primary.to_ascii_lowercase()
}

/// Read `LC_ALL` / `LC_MESSAGES` / `LANG` into a shipped locale tag.
pub fn system_locale() -> Option<String> {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(value) = std::env::var(key) {
            let value = value.trim();
            if value.is_empty()
                || value.eq_ignore_ascii_case("c")
                || value.eq_ignore_ascii_case("posix")
            {
                continue;
            }
            return Some(negotiate(value));
        }
    }
    None
}

fn tui_bundle(tag: &str) -> &'static Bundle {
    match negotiate(tag).as_str() {
        "es" => ES_TUI_BUNDLE.get_or_init(|| load_bundle("es", &[ES_COMMON, ES_TUI])),
        _ => EN_TUI_BUNDLE.get_or_init(|| load_bundle("en", &[EN_COMMON, EN_TUI])),
    }
}

fn desktop_bundle(tag: &str) -> &'static Bundle {
    match negotiate(tag).as_str() {
        "es" => ES_DESKTOP_BUNDLE.get_or_init(|| load_bundle("es", &[ES_COMMON, ES_DESKTOP])),
        _ => EN_DESKTOP_BUNDLE.get_or_init(|| load_bundle("en", &[EN_COMMON, EN_DESKTOP])),
    }
}

fn load_bundle(tag: &str, sources: &[&str]) -> Bundle {
    let lang: LanguageIdentifier = tag.parse().unwrap_or_else(|_| "en".parse().unwrap());
    let mut bundle = FluentBundle::new_concurrent(vec![lang]);
    bundle.set_use_isolating(false);
    if let Err(err) = bundle.add_builtins() {
        eprintln!("fluent builtins warning ({tag}): {err}");
    }

    for source in sources {
        add_resource(&mut bundle, tag, source);
    }

    bundle
}

fn add_resource(bundle: &mut Bundle, tag: &str, source: &str) {
    let resource = match FluentResource::try_new((*source).to_owned()) {
        Ok(res) => res,
        Err((res, errors)) => {
            for err in errors {
                eprintln!("fluent parse warning ({tag}): {err}");
            }
            res
        }
    };

    if let Err(errors) = bundle.add_resource(resource) {
        for err in errors {
            eprintln!("fluent resource warning ({tag}): {err}");
        }
    }
}

fn format_message(
    bundle: &Bundle,
    tag: &str,
    id: &str,
    args: Option<&FluentArgs<'_>>,
) -> Cow<'static, str> {
    let Some(message) = bundle.get_message(id) else {
        return Cow::Owned(id.to_owned());
    };
    let Some(pattern) = message.value() else {
        return Cow::Owned(id.to_owned());
    };

    let mut errors = Vec::new();
    let value = bundle.format_pattern(pattern, args, &mut errors);
    for err in errors {
        eprintln!("fluent format warning ({tag}/{id}): {err}");
    }
    Cow::Owned(value.into_owned())
}

fn format_attr(
    bundle: &Bundle,
    tag: &str,
    id: &str,
    attr: &str,
    args: Option<&FluentArgs<'_>>,
) -> Cow<'static, str> {
    let Some(message) = bundle.get_message(id) else {
        return Cow::Owned(format!("{id}.{attr}"));
    };
    let Some(attribute) = message.get_attribute(attr) else {
        return Cow::Owned(format!("{id}.{attr}"));
    };

    let mut errors = Vec::new();
    let value = bundle.format_pattern(attribute.value(), args, &mut errors);
    for err in errors {
        eprintln!("fluent format warning ({tag}/{id}.{attr}): {err}");
    }
    Cow::Owned(value.into_owned())
}

/// Format a TUI / shared message value for `tag`.
pub fn t<'a>(tag: &str, id: &str, args: Option<&'a FluentArgs<'a>>) -> Cow<'static, str> {
    format_message(tui_bundle(tag), tag, id, args)
}

/// Format a TUI / shared message attribute (e.g. `theme-toggle.label`).
pub fn t_attr<'a>(
    tag: &str,
    id: &str,
    attr: &str,
    args: Option<&'a FluentArgs<'a>>,
) -> Cow<'static, str> {
    format_attr(tui_bundle(tag), tag, id, attr, args)
}

/// Format a desktop message value for `tag` (common + desktop.ftl).
pub fn t_desktop<'a>(tag: &str, id: &str, args: Option<&'a FluentArgs<'a>>) -> Cow<'static, str> {
    format_message(desktop_bundle(tag), tag, id, args)
}

/// Format a desktop message attribute.
pub fn t_attr_desktop<'a>(
    tag: &str,
    id: &str,
    attr: &str,
    args: Option<&'a FluentArgs<'a>>,
) -> Cow<'static, str> {
    format_attr(desktop_bundle(tag), tag, id, attr, args)
}

/// Args commonly used by the welcome / showcase surfaces.
pub fn showcase_args<'a>(name: &'a str, date: &'a str) -> FluentArgs<'a> {
    let mut args = FluentArgs::new();
    args.set("name", FluentValue::from(name));
    args.set("count", FluentValue::from(3_i64));
    args.set("gigabytes", FluentValue::from(1.5_f64));
    args.set("date", FluentValue::from(date));
    args
}

/// Format today's date for the active locale (host-side stand-in for DATETIME).
pub fn format_session_date(tag: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return String::new();
    };
    let days = duration.as_secs() / 86_400;
    // Civil date from Unix day count (proleptic Gregorian).
    let (y, m, d) = civil_from_days(i64::try_from(days).unwrap_or(0));
    let month = month_name(tag, m);
    match tag {
        "es" => format!("{d} de {month} de {y}"),
        _ => format!("{month} {d}, {y}"),
    }
}

fn month_name(tag: &str, month: u32) -> &'static str {
    const EN: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    const ES: [&str; 12] = [
        "enero",
        "febrero",
        "marzo",
        "abril",
        "mayo",
        "junio",
        "julio",
        "agosto",
        "septiembre",
        "octubre",
        "noviembre",
        "diciembre",
    ];
    let idx = month.saturating_sub(1) as usize;
    match tag {
        "es" => ES.get(idx).copied().unwrap_or(""),
        _ => EN.get(idx).copied().unwrap_or(""),
    }
}

/// Howard's civil_from_days (days since Unix epoch → y/m/d).
fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = (yoe as i64 + era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negotiate_primary_and_fallback() {
        assert_eq!(negotiate("es-MX.UTF-8"), "es");
        assert_eq!(negotiate("en_US"), "en");
        assert_eq!(negotiate("fr"), "en");
    }

    #[test]
    fn cycle_rotates_available() {
        assert_eq!(cycle("en"), "es");
        assert_eq!(cycle("es"), "en");
    }

    #[test]
    fn formats_shared_and_tui_messages() {
        assert_eq!(t("en", "app-brand", None).as_ref(), "Mediaar");
        assert_eq!(t("es", "tui-tagline", None).as_ref(), "Gestiona tu media");
        assert!(t("en", "showcase-nested", None).contains("Press l"));
    }

    #[test]
    fn formats_number_builtin() {
        let date = format_session_date("en");
        let args = showcase_args("Alex", &date);
        let value = t("en", "showcase-number", Some(&args));
        assert!(value.contains("1.5"), "{value}");
    }

    #[test]
    fn formats_desktop_welcome() {
        assert_eq!(t_desktop("en", "welcome-title", None).as_ref(), "Welcome");
        assert!(
            t_desktop("en", "showcase-nested", None).contains("language control"),
            "{}",
            t_desktop("en", "showcase-nested", None)
        );
    }
}
