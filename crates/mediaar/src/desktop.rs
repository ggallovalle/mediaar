//! In-process Tauri desktop shell embedded in the `mediaar` binary.

use std::sync::Mutex;

use serde::Serialize;
use tauri::State;

use crate::i18n::{self, Locale};
use crate::settings;

#[derive(Debug)]
struct LocaleState {
    inner: Mutex<Locale>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocaleInfo {
    locale: String,
    available: Vec<String>,
    from_cli: bool,
}

#[tauri::command]
fn get_locale(state: State<'_, LocaleState>) -> Result<LocaleInfo, String> {
    let locale = state
        .inner
        .lock()
        .map_err(|_| "locale state poisoned".to_owned())?;
    Ok(LocaleInfo {
        locale: locale.tag.clone(),
        available: i18n::AVAILABLE.iter().map(|s| (*s).to_owned()).collect(),
        from_cli: locale.from_cli,
    })
}

#[tauri::command]
fn set_locale(locale: String, state: State<'_, LocaleState>) -> Result<LocaleInfo, String> {
    let tag = i18n::negotiate(&locale);
    settings::save_lang(&tag)?;

    let mut current = state
        .inner
        .lock()
        .map_err(|_| "locale state poisoned".to_owned())?;
    current.tag = tag.clone();
    current.from_cli = false;

    Ok(LocaleInfo {
        locale: tag,
        available: i18n::AVAILABLE.iter().map(|s| (*s).to_owned()).collect(),
        from_cli: false,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(locale: Locale) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(LocaleState {
            inner: Mutex::new(locale),
        })
        .invoke_handler(tauri::generate_handler![get_locale, set_locale])
        .run(tauri::generate_context!())
        .expect("error while running Mediaar desktop");
}
