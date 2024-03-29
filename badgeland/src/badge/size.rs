use crate::SizeError;
use std::{fmt, str::FromStr};

#[cfg(feature = "serde_de")]
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub enum Size {
    Large,
    Medium,
    #[default]
    Small,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Size::Large => "Large",
            Size::Medium => "Medium",
            Size::Small => "Small",
        };
        write!(f, "{}", s)
    }
}

#[cfg(feature = "serde_de")]
impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Size::from_str(s.as_str()).map_err(de::Error::custom)
    }
}

impl FromStr for Size {
    type Err = SizeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "large" | "l" => Ok(Size::Large),
            "medium" | "m" => Ok(Size::Medium),
            "small" | "s" => Ok(Size::Small),
            _ => Err(SizeError {}),
        }
    }
}
