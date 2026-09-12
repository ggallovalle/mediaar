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
const SHIPPED: &[&str] = &["en", "es"];

type Bundle = FluentBundle<FluentResource>;

static EN_TUI_BUNDLE: OnceLock<Bundle> = OnceLock::new();
static ES_TUI_BUNDLE: OnceLock<Bundle> = OnceLock::new();
static EN_DESKTOP_BUNDLE: OnceLock<Bundle> = OnceLock::new();
static ES_DESKTOP_BUNDLE: OnceLock<Bundle> = OnceLock::new();

pub fn fallback() -> LanguageIdentifier {
    langid("en")
}

fn langid(tag: &str) -> LanguageIdentifier {
    tag.parse().unwrap_or_else(|_| {
        "en".parse()
            .expect("en is a well-formed language identifier")
    })
}

/// Locales shipped with the binary.
pub fn shipped() -> &'static [&'static str] {
    SHIPPED
}

/// Catalog key used to pick Fluent resources (`en` or `es`).
pub fn catalog_key(id: &LanguageIdentifier) -> &'static str {
    match id.language.as_str() {
        "es" => "es",
        _ => "en",
    }
}

/// Next shipped locale in the cycle (for UI / TUI toggles).
pub fn cycle(current: &LanguageIdentifier) -> LanguageIdentifier {
    let key = catalog_key(current);
    let idx = SHIPPED.iter().position(|tag| *tag == key).unwrap_or(0);
    langid(SHIPPED[(idx + 1) % SHIPPED.len()])
}

fn parse_env_tag(raw: &str) -> Option<LanguageIdentifier> {
    let trimmed = raw.trim();
    if trimmed.is_empty()
        || trimmed.eq_ignore_ascii_case("c")
        || trimmed.eq_ignore_ascii_case("posix")
    {
        return None;
    }
    let without_encoding = trimmed.split('.').next().unwrap_or(trimmed);
    let without_modifier = without_encoding
        .split('@')
        .next()
        .unwrap_or(without_encoding);
    without_modifier.parse().ok()
}

/// Read `LC_ALL` / `LC_MESSAGES` / `LANG` into a language identifier.
pub fn system_locale() -> Option<LanguageIdentifier> {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(value) = std::env::var(key)
            && let Some(id) = parse_env_tag(&value)
        {
            return Some(id);
        }
    }
    None
}

fn tui_bundle(id: &LanguageIdentifier) -> &'static Bundle {
    match catalog_key(id) {
        "es" => ES_TUI_BUNDLE.get_or_init(|| load_bundle("es", &[ES_COMMON, ES_TUI])),
        _ => EN_TUI_BUNDLE.get_or_init(|| load_bundle("en", &[EN_COMMON, EN_TUI])),
    }
}

fn desktop_bundle(id: &LanguageIdentifier) -> &'static Bundle {
    match catalog_key(id) {
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

/// Format a TUI / shared message value for `lang`.
pub fn t<'a>(
    lang: &LanguageIdentifier,
    id: &str,
    args: Option<&'a FluentArgs<'a>>,
) -> Cow<'static, str> {
    let key = catalog_key(lang);
    format_message(tui_bundle(lang), key, id, args)
}

/// Format a TUI / shared message attribute (e.g. `theme-toggle.label`).
pub fn t_attr<'a>(
    lang: &LanguageIdentifier,
    id: &str,
    attr: &str,
    args: Option<&'a FluentArgs<'a>>,
) -> Cow<'static, str> {
    let key = catalog_key(lang);
    format_attr(tui_bundle(lang), key, id, attr, args)
}

/// Format a desktop message value for `lang` (common + desktop.ftl).
pub fn t_desktop<'a>(
    lang: &LanguageIdentifier,
    id: &str,
    args: Option<&'a FluentArgs<'a>>,
) -> Cow<'static, str> {
    let key = catalog_key(lang);
    format_message(desktop_bundle(lang), key, id, args)
}

/// Format a desktop message attribute.
pub fn t_attr_desktop<'a>(
    lang: &LanguageIdentifier,
    id: &str,
    attr: &str,
    args: Option<&'a FluentArgs<'a>>,
) -> Cow<'static, str> {
    let key = catalog_key(lang);
    format_attr(desktop_bundle(lang), key, id, attr, args)
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
pub fn format_session_date(lang: &LanguageIdentifier) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return String::new();
    };
    let days = duration.as_secs() / 86_400;
    // Civil date from Unix day count (proleptic Gregorian).
    let (y, m, d) = civil_from_days(i64::try_from(days).unwrap_or(0));
    let key = catalog_key(lang);
    let month = month_name(key, m);
    match key {
        "es" => format!("{d} de {month} de {y}"),
        _ => format!("{month} {d}, {y}"),
    }
}

fn month_name(key: &str, month: u32) -> &'static str {
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
    match key {
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
    fn parse_env_strips_encoding() {
        let es_mx = parse_env_tag("es-MX.UTF-8").unwrap();
        assert_eq!(es_mx.language.as_str(), "es");
        assert_eq!(parse_env_tag("en_US").unwrap().language.as_str(), "en");
        assert!(parse_env_tag("C").is_none());
    }

    #[test]
    fn cycle_rotates_shipped() {
        let en = langid("en");
        let es = langid("es");
        assert_eq!(cycle(&en), es);
        assert_eq!(cycle(&es), en);
        assert_eq!(cycle(&langid("es-MX")), en);
    }

    #[test]
    fn formats_shared_and_tui_messages() {
        let en = langid("en");
        let es = langid("es");
        assert_eq!(t(&en, "app-brand", None).as_ref(), "Mediaar");
        assert_eq!(t(&es, "tui-tagline", None).as_ref(), "Gestiona tu media");
        assert!(t(&en, "showcase-nested", None).contains("Press l"));
    }

    #[test]
    fn formats_number_builtin() {
        let en = langid("en");
        let date = format_session_date(&en);
        let args = showcase_args("Alex", &date);
        let value = t(&en, "showcase-number", Some(&args));
        assert!(value.contains("1.5"), "{value}");
    }

    #[test]
    fn formats_desktop_welcome() {
        let en = langid("en");
        assert_eq!(t_desktop(&en, "welcome-title", None).as_ref(), "Welcome");
        assert!(
            t_desktop(&en, "showcase-nested", None).contains("Open Settings"),
            "{}",
            t_desktop(&en, "showcase-nested", None)
        );
    }
}
