use super::get_color;
use maud::{html, PreEscaped, Render};
use std::{convert, str};

include!(concat!(env!("OUT_DIR"), "/icons_map.rs"));

static DEFAULT_COLOUR: &str = "#fff";

pub fn icon_exists(icon_name: &str) -> bool {
  SYMBOLS.get(icon_name).is_some()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Icon<'a> {
  pub name: &'a str,
  pub color: String,
  pub symbol: String,
}

impl<'a> Icon<'a> {
  pub fn new(name: &'a str) -> Self {
    Icon {
      name,
      color: DEFAULT_COLOUR.into(),
      symbol: "".into(),
    }
  }
  pub fn color(&mut self, value: impl convert::Into<String>) -> &mut Self {
    self.color = value.into();
    self
  }
  pub fn build(&self) -> Option<Icon<'a>> {
    let Icon { name, color, symbol: _ } = self;

    let color = get_color(&color)?;
    match SYMBOLS.get(*name) {
      Some(symbol) => Some(Icon {
        name,
        color,
        symbol: String::from(*symbol),
      }),
      _ => None,
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
  use super::Icon;

  #[test]
  fn get_icon_symbol() {
    let icon = Icon::new("bluetooth-b").build();
    assert!(icon.is_some());
    assert!(icon.unwrap().symbol.len() > 0);
  }

  #[test]
  fn get_icon_with_color() {
    let icon = Icon::new("bluetooth-b").color("red").build();
    assert!(icon.is_some());
    assert_eq!(icon.unwrap().color, "rgb(255, 0, 0)".to_string());
  }
}
