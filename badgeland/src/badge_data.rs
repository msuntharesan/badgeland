use std::{convert::From, num::ParseFloatError, str::FromStr};

#[cfg(feature = "serde_de")]
use serde::Deserialize;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde_de", derive(Deserialize))]
pub struct BadgeData(pub Vec<f32>);

impl FromStr for BadgeData {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(",")
            .map(|s| s.trim().parse::<f32>())
            .collect::<Result<Vec<_>, Self::Err>>()
            .map(|values| BadgeData(values))
    }
}

impl From<Vec<f32>> for BadgeData {
    fn from(values: Vec<f32>) -> Self {
        BadgeData(values)
    }
}

#[cfg(test)]
mod tests {

    use super::BadgeData;

    #[test]
    fn data_from_string_fails() {
        let d = "not a number".parse::<BadgeData>();
        assert!(d.is_err());
        let d = "12,12,,12".parse::<BadgeData>();
        assert!(d.is_err());
    }

    #[test]
    fn data_from_string_parse_correct() {
        let d = "12,23, 23, 12".parse::<BadgeData>();
        assert!(d.is_ok());
        assert_eq!(d.unwrap().0, vec![12., 23., 23., 12.]);
    }

    #[test]
    fn data_from_json_parse_fails() {
        let d = "12, 32!,23, 23, 12".parse::<BadgeData>();
        assert!(d.is_err());
    }
}
