use std::{iter::FromIterator, num::ParseFloatError, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde_de", derive(serde::Serialize, serde::Deserialize))]
pub struct BadgeData(pub Vec<f32>);

impl FromStr for BadgeData {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(",")
            .map(|s| s.trim().parse::<f32>())
            .collect::<Result<Self, Self::Err>>()
    }
}

impl AsRef<[f32]> for BadgeData {
    fn as_ref(&self) -> &[f32] {
        &self.0[..]
    }
}

impl FromIterator<f32> for BadgeData {
    fn from_iter<I: IntoIterator<Item = f32>>(iter: I) -> Self {
        BadgeData(iter.into_iter().collect())
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
    fn struct_collect_pass() {
        let d: BadgeData = vec![12, 32, 32, 12, 42].into_iter().map(|v| v as f32).collect();
        assert_eq!(d, BadgeData(vec![12., 32., 32., 12., 42.]));
    }
}
