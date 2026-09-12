use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use unic_langid::LanguageIdentifier;
use usage::config::{FromValue, TypeError, Value};

/// Unicode language identifier, readable from usage-config strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lang(LanguageIdentifier);

impl Deref for Lang {
    type Target = LanguageIdentifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<LanguageIdentifier> for Lang {
    fn from(value: LanguageIdentifier) -> Self {
        Self(value)
    }
}

impl From<Lang> for LanguageIdentifier {
    fn from(value: Lang) -> Self {
        value.0
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Lang {
    type Err = unic_langid::LanguageIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl FromValue for Lang {
    fn from_value(value: &Value) -> Result<Self, TypeError> {
        match value {
            Value::String(raw) => raw.parse().map_err(|_| TypeError {
                expected: "a language identifier",
                found: raw.clone(),
            }),
            other => Err(TypeError {
                expected: "a language identifier",
                found: other.to_string(),
            }),
        }
    }
}
