// use phf_codegen::Map;
use phf_codegen::Map;
use scraper::{Html, Selector};
use std::fs::File;
use std::{
  env,
  io::{BufWriter, Write},
  path::Path,
};

fn generate_icon_map() {
  let path = Path::new(&env::var("OUT_DIR").unwrap()).join("icons_map.rs");
  let selector = Selector::parse("symbol").unwrap();

  let mut file = BufWriter::new(File::create(path).unwrap());

  let mut map = Map::<&'static str>::new();

  let doc = Html::parse_fragment(include_str!("./icons/brands.svg"));

  for el in doc.select(&selector) {
    let id = el.value().attr("id").unwrap();
    let sym = el.html();
    if id == "font-awesome-logo-full" {
      continue;
    }
    map.entry(id, &format!(r##"r#"{}"#"##, sym));
  }

  let doc = Html::parse_fragment(include_str!("./icons/solid.svg"));
  for el in doc.select(&selector) {
    let id = el.value().attr("id").unwrap();
    let sym = el.html();
    if id == "font-awesome-logo-full" {
      continue;
    }
    map.entry(id, &format!(r##"r#"{}"#"##, sym));
  }

  writeln!(
    &mut file,
    "const SYMBOLS: phf::Map<&'static str, &'static str> = {};",
    map.build()
  )
  .expect("Failed to build icon map");
}

fn main() {
  generate_icon_map()
}
