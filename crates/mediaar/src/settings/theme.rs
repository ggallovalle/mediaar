use std::fmt;
use std::str::FromStr;

use usage::config::{FromValue, TypeError, Value};

/// Light, dark, or follow the OS — same meaning as Zed's Theme Mode / Mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeAppearanceMode {
    Light,
    #[default]
    Dark,
    System,
}

impl ThemeAppearanceMode {
    pub const ALL: [Self; 3] = [Self::Light, Self::Dark, Self::System];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }
}

impl fmt::Display for ThemeAppearanceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ThemeAppearanceMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "system" => Ok(Self::System),
            _ => Err(()),
        }
    }
}

impl FromValue for ThemeAppearanceMode {
    fn from_value(value: &Value) -> Result<Self, TypeError> {
        match value {
            Value::String(raw) => raw.parse().map_err(|_| TypeError {
                expected: "light, dark, or system",
                found: raw.clone(),
            }),
            other => Err(TypeError {
                expected: "light, dark, or system",
                found: other.to_string(),
            }),
        }
    }
}
