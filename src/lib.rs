#![feature(proc_macro_hygiene)]

use cssparser::{parse_color_keyword, Color, ToCss};
use std::{num::ParseIntError, str::FromStr};

mod badge;
mod icons;

pub use badge::{Badge, Size, Styles};
pub use icons::{icon_exists, Icon, IconBuilder};

pub(self) fn get_color(color: &str) -> Option<String> {
  let mut color = color.to_string();
  if color.starts_with("#") {
    color = color.replace("#", "");
  }
  match (
    Color::parse_hash(color.as_bytes()),
    parse_color_keyword(&color),
  ) {
    (Ok(c), _) => Some(c.to_css_string()),
    (_, Ok(c)) => Some(c.to_css_string()),
    (_, _) => None,
  }
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
  fn get_color_from_name() {
    let c = get_color("red");
    assert_eq!(c, Some(String::from("rgb(255, 0, 0)")));
  }
  #[test]
  fn get_color_from_hex() {
    let c = get_color("ff0000");
    assert_eq!(c, Some(String::from("rgb(255, 0, 0)")));
  }
  #[test]
  fn get_color_from_hex_with_prefix() {
    let c = get_color("#ff0000");
    assert_eq!(c, Some(String::from("rgb(255, 0, 0)")));
  }
  #[test]
  fn get_color_fails() {
    let c = get_color("nonexistant");
    assert!(c.is_none());
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
