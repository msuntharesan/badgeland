#[cfg(feature = "serde_de")]
use serde::{Deserialize, Serialize};
use std::{iter::FromIterator, num::ParseFloatError, str::FromStr, sync::Arc};

#[derive(Debug, PartialEq, Clone)]
pub struct BadgeData(pub Arc<[f32]>);

#[cfg(feature = "serde_de")]
impl Serialize for BadgeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde_de")]
impl<'de> Deserialize<'de> for BadgeData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data: Vec<f32> = Vec::deserialize(deserializer)?;
        Ok(BadgeData(Arc::from(data.into_boxed_slice())))
    }
}

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
        assert_eq!(d.unwrap().0, vec![12., 23., 23., 12.].into());
    }

    #[test]
    fn struct_collect_pass() {
        let d: BadgeData = vec![12, 32, 32, 12, 42]
            .into_iter()
            .map(|v| v as f32)
            .collect();
        assert_eq!(d, BadgeData(vec![12., 32., 32., 12., 42.].into()));
    }

    #[cfg(feature = "serde_de")]
    mod serde_tests {
        use super::BadgeData;
        use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

        #[test]
        fn struct_serialize_pass() {
            let d = BadgeData(vec![12., 32., 32., 12., 42.].into());
            assert_ser_tokens(
                &d,
                &[
                    Token::Seq { len: Some(5) },
                    Token::F32(12.),
                    Token::F32(32.),
                    Token::F32(32.),
                    Token::F32(12.),
                    Token::F32(42.),
                    Token::SeqEnd,
                ],
            );
        }

        #[test]
        fn struct_deserialize_pass() {
            let d = BadgeData(vec![12., 32., 32., 12., 42.].into());
            assert_de_tokens(
                &d,
                &[
                    Token::Seq { len: Some(5) },
                    Token::F32(12.),
                    Token::F32(32.),
                    Token::F32(32.),
                    Token::F32(12.),
                    Token::F32(42.),
                    Token::SeqEnd,
                ],
            );
        }
    }
}
