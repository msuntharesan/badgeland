//! Fast badge generator for any purpose
//!
//! Create badges with text, icons and sparkline chart
//!
//! # Web
//!
//! See <https://github.com/msuntharesan/badgeland#web>
//!
//! # Quick start
//!
//! Add `badgeland` to your `Cargo.toml` as as a dependency.
//!
//! # Examples
//!
//! ```rust
//! use badgeland::{Badge};
//!
//! fn badge() {
//!   let mut badge = Badge::new();
//!   badge.subject("Subject");
//!   println!("{}", badge.text("Text").to_string());
//! }
//! ```
//! This produce a svg badge: ![](https://badge.land/b/Subject/Text)
//!```rust
//! use badgeland::{Badge};
//!
//! fn badge_with_data() {
//!   let mut badge = Badge::new();
//!   badge.subject("Subject");
//!   println!("{}", badge.data(&[12., 34., 23., 56., 45.]).to_string());
//! }
//! ```
//! This produce a svg badge: ![](http://badge.land/b/testing/12,34,23,56,45)
//!

use cssparser::{Color as CssColor, Parser, ParserInput, ToCss};
use std::{convert::From, num::ParseFloatError, str::FromStr};

#[cfg(feature = "serde_de")]
use serde::{de, Deserialize, Deserializer, Serialize};

mod badge;
mod icons;

pub use badge::{Badge, Size, Style};
pub use icons::{icon_exists, icon_keys, Icon};

pub type InitialBadge<'a> = Badge<'a, badge::BadgeTypeInit>;

pub const DEFAULT_WHITE: &'static str = "#fff";
pub const DEFAULT_BLACK: &'static str = "#000";
pub const DEFAULT_BLUE: &'static str = "#0366d6";
pub const DEFAULT_GRAY: &'static str = "#f6f8fa";
pub const DEFAULT_GRAY_DARK: &'static str = "#24292e";

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub struct Color(pub String);

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);

        match (CssColor::parse(&mut parser), CssColor::parse_hash(s.as_bytes())) {
            (Ok(c), _) | (_, Ok(c)) => Ok(Color(c.to_css_string())),
            _ => Err(format!("{} is not a valid css color", s)),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::from_str("#000").unwrap()
    }
}

#[cfg(feature = "serde_de")]
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Color::from_str(&s).map_err(de::Error::custom)
    }
}

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

    use super::{BadgeData, Color};
    use std::str::FromStr;

    #[test]
    fn get_color_pass() {
        let colors = vec!["red", "#ff0000", "ff0000", "rgb(255, 0, 0)", "rgba(255, 0, 0, 1)"];

        let expected = Ok(Color(String::from("rgb(255, 0, 0)")));

        for c in colors {
            let cx = Color::from_str(c);
            assert_eq!(
                cx, expected,
                "input = {}, received = {:?}, expected = {:?}",
                c, cx, expected
            );
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
        }
    }

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
    fn dat_from_json_parse_fails() {}
}
