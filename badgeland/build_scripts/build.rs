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

  let mut file = BufWriter::new(File::create(path).unwrap());

  let mut map = Map::<&str>::new();

  let exclude_list = [
    "anchor",
    "atom",
    "blender",
    "box",
    "circle",
    "flask",
    "ghost",
    "handshake",
    "meteor",
    "ring",
    "rss",
    "signal",
    "snowflake",
    "square",
    "thumbtack",
  ];

  let selector = Selector::parse("symbol").unwrap();

  let doc = Html::parse_fragment(include_str!("./icons/solid.svg"));
  for el in doc.select(&selector) {
    let id = el.value().attr("id").unwrap();
    let sym = el.html();
    if !exclude_list.contains(&id) {
      map.entry(id, &format!(r##"r#"{}"#"##, sym));
    }
  }

  let doc = Html::parse_fragment(include_str!("./icons/simple-icons.svg"));
  for el in doc.select(&selector) {
    let id = el.value().attr("id").unwrap();
    let sym = el.html();
    if !exclude_list.contains(&id) {
      map.entry(id, &format!(r##"r#"{}"#"##, sym));
    }
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
