use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

/// Embedded `ui` icon SVGs (Zed `icons/` layout, subset).
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let bytes: &[u8] = match path {
            "icons/check.svg" => include_bytes!("../icons/check.svg"),
            "icons/chevron_down.svg" => include_bytes!("../icons/chevron_down.svg"),
            "icons/chevron_right.svg" => include_bytes!("../icons/chevron_right.svg"),
            "icons/chevron_up_down.svg" => include_bytes!("../icons/chevron_up_down.svg"),
            "icons/magnifying_glass.svg" => include_bytes!("../icons/magnifying_glass.svg"),
            _ => return Ok(None),
        };
        Ok(Some(Cow::Borrowed(bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        if path == "icons" || path == "icons/" {
            Ok(vec![
                "check.svg".into(),
                "chevron_down.svg".into(),
                "chevron_right.svg".into(),
                "chevron_up_down.svg".into(),
                "magnifying_glass.svg".into(),
            ])
        } else {
            Ok(Vec::new())
        }
    }
}
