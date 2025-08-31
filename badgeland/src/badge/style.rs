use crate::StyleError;
use std::{fmt, str::FromStr};

#[cfg(feature = "serde_de")]
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub enum Style {
    #[default]
    Classic,
    Flat,
    Social,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Style::Classic => "Classic",
            Style::Flat => "Flat",
            Style::Social => "Social",
        };
        write!(f, "{}", s)
    }
}

#[cfg(feature = "serde_de")]
impl<'de> Deserialize<'de> for Style {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Style::from_str(&s).map_err(|e| de::Error::custom(e))
    }
}

impl FromStr for Style {
    type Err = StyleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "classic" | "c" => Ok(Style::Classic),
            "flat" | "f" => Ok(Style::Flat),
            "social" | "s" => Ok(Style::Social),
            _ => Err(Self::Err {}),
        }
    }
}
