use cssparser::{Color as CssColor, Parser, ParserInput, ToCss};
use std::{fmt::Display, str::FromStr};

#[cfg(feature = "serde_de")]
use serde::{de, Deserialize, Deserializer, Serialize};

use super::error::ColorError;

pub const DEFAULT_WHITE: &'static str = "#fff";
pub const DEFAULT_BLACK: &'static str = "#000";
pub const DEFAULT_BLUE: &'static str = "#0366d6";
pub const DEFAULT_GRAY: &'static str = "#f6f8fa";
pub const DEFAULT_GRAY_DARK: &'static str = "#24292e";

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub struct Color(pub String);

impl FromStr for Color {
    type Err = ColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);

        CssColor::parse(&mut parser)
            .or_else(|_| CssColor::parse_hash(s.as_bytes()))
            .map(|c| Color(c.to_css_string()))
            .map_err(|_| Self::Err {})
    }
}

impl Default for Color {
    fn default() -> Self {
        "#000".parse().unwrap()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(feature = "serde_de")]
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.as_str().parse().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use super::Color;
    use std::str::FromStr;

    #[test]
    fn get_color_pass() {
        let colors = vec![
            "red",
            "#ff0000",
            "ff0000",
            "rgb(255, 0, 0)",
            "rgba(255, 0, 0, 1)",
        ];

        let expected = Color(String::from("rgb(255, 0, 0)"));

        for c in colors {
            let cx = Color::from_str(c);
            assert!(cx.is_ok(), "input = {}, received = {:?}", c, cx);

            let cx = cx.unwrap();

            assert_eq!(
                cx, expected,
                "input = {}, received = {:?}, expected = {:?}",
                c, cx, expected
            )
        }
    }
    #[test]
    fn get_color_fail() {
        let colors = vec![
            "2983492837498723",
            "mixed",
            "#gg0000",
            "gg0000",
            "rbx(adf, 0, 0)",
            "rgba(ee0, 0, 0, 1)",
        ];

        for c in colors {
            let cx = Color::from_str(c);

            assert!(cx.is_err(), "input = {}, received = {:?}", c, cx);
            assert_eq!(cx.unwrap_err().to_string(), "Invalid Color".to_string());
        }
    }
}
