//! Fast badge generator for any purpose
//!
//! Create badges with text, icons and sparkline chart
//!
//! # Web
//!
//! See <https://github.com/msuntharesan/merit#web>
//!
//! # Cli
//!
//! Install using `cargo install merit`
//!
//! ```
//! USAGE:
//!     merit [OPTIONS] --subject <subject>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     --color <color>                6 or 8 digit hex color or a valid css color name
//!     --data <data>
//!     --icon <icon>                  Icon cany be any Brand or Solid icons from fontawesome
//!     --icon-colour <icon-colour>    6 or 8 digit hex color or a valid css color name
//!     --out <out>
//!     --size <size>                   [possible values: Large, Medium, Small]
//!     --style <style>                 [possible values: Flat, Classic]
//!     --subject <subject>
//!     --text <text>
//! ```
//!
//! # Quick start
//!
//! Add `merit` to your `Cargo.toml` as as a dependency.
//!
//! # Examples
//!
//! ```rust
//! use merit::{Badge};
//!
//! fn badge() {
//!   let mut badge = Badge::new("Subject");
//!   badge.text("Text");
//!   println!("{}", badge.to_string());
//! }
//! ```
//! This produce a svg badge: ![](https://merit-badge.appspot.com/badge/Subject/Text)
//!```rust
//! use merit::{Badge};
//!
//! fn badge_with_data() {
//!   let mut badge = Badge::new("Subject").data(vec![12, 34, 23,56,45]);
//!   println!("{}", badge.to_string());
//! }
//! ```
//! This produce a svg badge: ![](http://merit-badge.appspot.com/badge/testing/12,34,23,56,45)
//!

#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(proc_macro_hygiene)]

use cssparser::{Color, Parser, ParserInput, ToCss};
use std::{num::ParseIntError, str::FromStr};

mod badge;
mod icons;

pub use badge::{Badge, Size, Styles};
pub use icons::{icon_exists, Icon};

pub(self) fn get_color(color: &str) -> Option<String> {
  let mut input = ParserInput::new(color);
  let mut parser = Parser::new(&mut input);

  Color::parse(&mut parser)
    .map(|c: Color| c.to_css_string())
    .or_else(|_| Color::parse_hash(&color.as_bytes()).map(|c: Color| c.to_css_string()))
    .ok()
}

#[derive(Debug)]
pub struct BadgeData(pub Vec<i64>);

impl FromStr for BadgeData {
  type Err = ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.split(",")
      .map(|s| s.trim().parse::<i64>())
      .collect::<Result<Vec<_>, Self::Err>>()
      .map(|values| BadgeData(values))
  }
}

#[cfg(test)]
mod tests {

  use super::{get_color, BadgeData};

  #[test]
  fn get_color_pass() {
    let colors = vec!["red", "#ff0000", "ff0000", "rgb(255, 0, 0)", "rgba(255, 0, 0, 1)"];

    let expected = Some(String::from("rgb(255, 0, 0)"));

    for c in colors {
      let cx = get_color(c);
      assert_eq!(
        cx, expected,
        "input = {},  received = {:?}, expected = {:?}",
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
      let cx = get_color(c);

      assert!(
        cx.is_none(),
        "input = {},  received = {:?}, expected = {:?}",
        c,
        cx,
        None::<String>
      );
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
    assert_eq!(d.unwrap().0, vec![12, 23, 23, 12]);
  }
}
