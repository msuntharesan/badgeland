#![feature(proc_macro_hygiene)]

mod badge;
mod icons;

pub use badge::{Badge, Size, Styles};
use cssparser::{parse_color_keyword, Color, ToCss};
pub use icons::icon_exists;

pub(self) fn get_color(color: &str) -> Option<String> {
  match (
    Color::parse_hash(color.as_bytes()),
    parse_color_keyword(color),
  ) {
    (Ok(c), _) => Some(c.to_css_string()),
    (_, Ok(c)) => Some(c.to_css_string()),
    (_, _) => None,
  }
}

#[cfg(test)]
mod tests {

  use super::get_color;

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
  fn get_color_fails() {
    let c = get_color("nonexistant");
    assert!(c.is_none());
  }
}