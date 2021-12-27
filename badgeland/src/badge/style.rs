use crate::StyleError;
use std::str::FromStr;

#[cfg(feature = "serde_de")]
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub enum Style {
    Classic,
    Flat,
}

impl Default for Style {
    fn default() -> Self {
        Style::Classic
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
            _ => Err(Self::Err {}),
        }
    }
}
