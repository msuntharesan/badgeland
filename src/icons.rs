use maud::{html, PreEscaped, Render};
use std::convert::TryFrom;

include!(concat!(env!("OUT_DIR"), "/icons_map.rs"));

pub fn icon_exists(icon_name: &str) -> bool {
  SYMBOLS.get(icon_name).is_some()
}

pub fn icon_keys() -> Vec<String> {
  SYMBOLS.keys().map(|k| String::from(*k)).collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Icon<'a> {
  pub name: &'a str,
  pub symbol: &'a str,
}

impl<'a> TryFrom<&'a str> for Icon<'a> {
  type Error = Box<dyn std::error::Error>;

  fn try_from(name: &'a str) -> Result<Self, Self::Error> {
    match SYMBOLS.get(name) {
      Some(symbol) => Ok(Icon { name, symbol }),
      _ => Err("Icon does not exists".into()),
    }
  }
}

impl<'a> Render for Icon<'a> {
  fn render(&self) -> maud::Markup {
    html! {
      defs {
        (PreEscaped(self.symbol.to_string()))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{icon_keys, Icon};
  use std::convert::TryFrom;

  #[test]
  fn get_icon_symbol() {
    let icon = Icon::try_from("bluetooth-b");
    assert!(icon.is_ok());
    assert!(icon.unwrap().symbol.len() > 0);
  }

  #[test]
  fn get_icon_keys() {
    assert!(icon_keys().len() > 0);
  }
}
