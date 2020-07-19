//! # Cli
//!
//! Install using `cargo install merit`
//!
//! ```sh
//! Usage: badge -s <subject> [--style <style>] [--size <size>] [--color <color>] [--icon <icon>] [--icon-color <icon-color>] [--out <out>] [-t <text>] [--data <data>]
//!
//! Fast badge generator for any purpose
//!
//! Options:
//!   -s, --subject     badge subject
//!   --style           badge style. [possible values: flat | f, classic | c]
//!   --size            badge size. [possible values: large | l, medium | m, small | s]
//!   --color           badge color. Must be a valid css color
//!   --icon            badge icon. icon can be any Brand or Solid icons from fontawesome
//!   --icon-color      icon color. Must be a valid css color
//!   --out             output svg to file
//!   -t, --text        badge text
//!   --data            data for badge chart.
//!   --help            display usage information
//! ```
//!

use argh::FromArgs;
use merit::{icon_exists, Badge, BadgeData, Color, Icon, Size, Styles};
use std::{convert::TryFrom, error::Error, fs::File, io::prelude::*, path::PathBuf};

#[derive(FromArgs)]
/// Fast badge generator for any purpose
struct Opt {
  /// badge subject
  #[argh(option, short = 's')]
  subject: String,

  /// badge style. [possible values: flat | f, classic | c]
  #[argh(option)]
  style: Option<Styles>,

  /// badge size. [possible values: large | l, medium | m, small | s]
  #[argh(option)]
  size: Option<Size>,

  /// badge color. Must be a valid css color
  #[argh(option)]
  color: Option<Color>,

  /// badge icon. icon can be any Brand or Solid icons from fontawesome
  #[argh(option)]
  icon: Option<String>,

  /// icon color. Must be a valid css color
  #[argh(option)]
  icon_color: Option<Color>,

  /// output svg to file
  #[argh(option)]
  out: Option<PathBuf>,

  /// badge text
  #[argh(option, short = 't')]
  text: Option<String>,

  /// data for badge chart.
  #[argh(option)]
  data: Option<BadgeData>,
}

fn main() -> Result<(), Box<dyn Error>> {
  let opt: Opt = argh::from_env();

  if let Some(icon) = &opt.icon {
    if !icon_exists(icon) {
      eprintln!("Icon does not exists. Try using a fontawesome icon name");
      std::process::exit(1);
    }
  }
  let mut badge = Badge::new(&opt.subject);

  if let Some(col) = opt.color {
    badge.color(col);
  }

  if let Some(s) = opt.style {
    badge.style(s);
  }
  if let Some(s) = opt.size {
    badge.size(s);
  }
  if let Some(icon) = &opt.icon {
    let icon = Icon::try_from(icon.as_str());
    if let Ok(i) = icon {
      badge.icon(i);
    }
    if let Some(c) = opt.icon_color {
      badge.icon_color(c);
    }
  }

  let svg = match (opt.data, opt.text) {
    (Some(d), _) => badge.data(d.0.into()).to_string(),
    (_, Some(t)) => badge.text(&t).to_string(),
    (_, _) => badge.to_string(),
  };

  if let Some(out_file) = opt.out {
    let mut file = File::create(&out_file).unwrap();
    file.write_all(svg.as_bytes()).unwrap();
  } else {
    println!("{}", svg);
  }
  Ok(())
}
