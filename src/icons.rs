use super::get_color;
use derive_builder::*;
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

    let selector = Selector::parse("symbol").unwrap();

    for src in sources.iter() {
      let doc = Html::parse_fragment(src);
      for el in doc.select(&selector) {
        let id = el.value().attr("id").unwrap().to_owned();
        let sym = el.html();
        symbols.insert(id, sym);
      }
    }

    symbols
  };
}

pub fn icon_exists(icon_name: &str) -> bool {
  SYMBOLS.contains_key(icon_name)
}

#[derive(Debug, PartialEq, Eq, Builder)]
#[builder(build_fn(skip), setter(prefix = "set"))]
pub struct Icon<'a> {
  #[builder(private)]
  pub name: &'a str,
  #[builder(setter(into))]
  pub color: String,
  #[builder(setter(skip))]
  pub symbol: String,
}

impl<'a> Icon<'a> {
  pub fn new(name: &'a str) -> IconBuilder {
    IconBuilder {
      name: Some(name),
      color: Some(DEFAULT_COLOUR.into()),
      ..IconBuilder::default()
    }
  }
}

impl<'a> IconBuilder<'a> {
  pub fn build(&self) -> Option<Icon<'a>> {
    let IconBuilder {
      name,
      color,
      symbol: _,
    } = &self;

    let name = name.unwrap();
    let color = color.as_ref().and_then(|c| get_color(&c)).unwrap();

    SYMBOLS.get(name).map(String::from).map(|symbol| Icon {
      name,
      color,
      symbol,
    })
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
    let icon = Icon::new("bluetooth-b").set_color("red").build();
    assert!(icon.is_some());
    assert_eq!(icon.unwrap().color, "rgb(255, 0, 0)".to_string());
  }
}
