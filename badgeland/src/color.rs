use cssparser::{Parser, ParserInput, ToCss};
use cssparser_color::Color as CssColor;
use sailfish::runtime::{Buffer, Render, RenderError};
use std::{borrow::Cow, convert::From, fmt::Display, str::FromStr};

#[cfg(feature = "serde_de")]
use serde::{de, Deserialize, Deserializer, Serialize};

use super::error::ColorError;

const DEFAULT_WHITE: &'static str = "rgb(255, 255, 255)";
const DEFAULT_BLACK: &'static str = "rgb(0, 0, 0)";
const DEFAULT_BLUE: &'static str = "rgb(3, 102, 214)";
const DEFAULT_GRAY: &'static str = "rgb(246, 248, 250)";
const DEFAULT_GRAY_DARK: &'static str = "rgb(36, 41, 46)";

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub struct Color(Cow<'static, str>);

impl Color {
    #[inline]
    pub fn white() -> Color {
        Color(DEFAULT_WHITE.into())
    }
    #[inline]
    pub fn black() -> Color {
        Color(DEFAULT_BLACK.into())
    }
    #[inline]
    pub fn blue() -> Color {
        Color(DEFAULT_BLUE.into())
    }
    #[inline]
    pub fn gray() -> Color {
        Color(DEFAULT_GRAY.into())
    }
    #[inline]
    pub fn gray_dark() -> Color {
        Color(DEFAULT_GRAY_DARK.into())
    }
}

impl FromStr for Color {
    type Err = ColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);

        CssColor::parse(&mut parser)
            .map_err(|_| Self::Err {})
            .and_then(|c| {
                let mut w = String::new();
                if matches!(c.to_css(&mut w), Err(_)) {
                    Err(Self::Err {})
                } else {
                    Ok(Color(w.into()))
                }
            })
    }
}

impl From<String> for Color {
    #[inline]
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
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

impl Render for Color {
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        let _ = self.0.render(b);
        Ok(())
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
            "#f00",
            "rgb(255, 0, 0)",
            "rgba(255, 0, 0, 1)",
        ];

        let expected = Color("rgb(255, 0, 0)".into());

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
