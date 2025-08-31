use phf_codegen::Map;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs::File;
use std::{
    env,
    io::{BufWriter, Write},
    path::Path,
};

fn generate_icon_map() {
    let mut map = Map::<&str>::new();
    let mut seen: HashSet<String> = HashSet::new();

    let selector = Selector::parse("symbol").unwrap();

    let doc = Html::parse_fragment(include_str!("./icons/solid.svg"));
    for el in doc.select(&selector) {
        let id = el.value().attr("id").unwrap();
        if !seen.insert(id.to_string()) {
            continue;
        }
        let sym = el.html();
        map.entry(id, format!(r##"r#"{}"#"##, sym));
    }

    let doc = Html::parse_fragment(include_str!("./icons/simple-icons.svg"));
    for el in doc.select(&selector) {
        let id = el.value().attr("id").unwrap();
        if !seen.insert(id.to_string()) {
            continue;
        }
        let sym = el.html();
        map.entry(id, format!(r##"r#"{}"#"##, sym));
    }

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("icons_map.rs");

    let mut file = BufWriter::new(File::create(path).unwrap());
    writeln!(
        &mut file,
        "const SYMBOLS: phf::Map<&'static str, &'static str> = {};",
        map.build()
    )
    .expect("Failed to build icon map");
}

fn main() {
    if cfg!(feature = "static_icons") {
        generate_icon_map()
    }
}
