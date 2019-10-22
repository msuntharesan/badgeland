use super::get_color;
use lazy_static::*;
use scraper::{Html, Selector};
use std::{collections::HashMap, str};

static DEFAULT_COLOUR: &str = "#fff";

lazy_static! {
  static ref SYMBOLS: HashMap<String, String> = {
    let mut symbols: HashMap<String, String> = HashMap::new();
    let sources = [
      include_str!("./resx/icons/brands.svg"),
      include_str!("./resx/icons/solid.svg"),
    ];
    for src in sources.iter() {
      let doc = Html::parse_fragment(src);
      let selector = Selector::parse("symbol").unwrap();
      for el in doc.select(&selector) {
        let id = el.value().attr("id").unwrap().to_owned();
        let sym = el.html();
        symbols.insert(id, sym);
      }
    }
    symbols
  };
}
fn get_symbol(name: &str) -> Option<String> {
  match SYMBOLS.get(name) {
    Some(s) => Some(s.to_owned()),
    None => None,
  }
}
pub fn icon_exists(icon_name: &str) -> bool {
  SYMBOLS.contains_key(icon_name)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Icon<'a> {
  pub name: &'a str,
  pub color: String,
  pub symbol: String,
}

pub struct IconBuilder<'a> {
  name: &'a str,
  color: &'a str,
  has_symbol: bool,
}

impl<'a> IconBuilder<'a> {
  pub fn new(name: &'a str) -> Self {
    IconBuilder {
      name,
      color: DEFAULT_COLOUR,
      has_symbol: icon_exists(name),
    }
  }
  pub fn set_color(&mut self, color: &'a str) -> &mut Self {
    self.color = color;
    self
  }
  pub fn build(self) -> Option<Icon<'a>> {
    if !self.has_symbol {
      return None;
    }
    Some(Icon {
      name: self.name,
      color: get_color(self.color).unwrap(),
      symbol: get_symbol(&self.name).unwrap(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::IconBuilder;

  #[test]
  fn get_icon_symbol() {
    let icon = IconBuilder::new("bluetooth-b").build();
    assert!(icon.is_some());
    assert!(icon.unwrap().symbol.len() > 0);
  }
}
