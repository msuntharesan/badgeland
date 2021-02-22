//! # Cli
//!
//! Install using `cargo install badgeland`
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

// use argh::FromArgs;
use clap::Clap;
use badgeland::{icon_exists, Badge, BadgeData, Color, Icon, Size, Style};
use std::{convert::TryFrom, error::Error, fs::File, io::prelude::*, path::PathBuf, str::FromStr};

#[derive(Debug, PartialEq)]
enum Content {
  Text(String),
  Data(BadgeData),
}

impl FromStr for Content {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(BadgeData::from_str(s).map_or(Content::Text(s.to_string()), |d| Content::Data(d)))
  }
}

/// Fast badge generator for any purpose
#[derive(Debug, Clap)]
#[clap(version)]
struct Opt {
  /// badge subject
  #[clap(short, long)]
  subject: Option<String>,

  /// badge style. [possible values: flat | f, classic | c]
  #[clap(long)]
  style: Option<Style>,

  /// badge size. [possible values: large | l, medium | m, small | s]
  #[clap(long)]
  size: Option<Size>,

  /// badge color. Must be a valid css color
  #[clap(long)]
  color: Option<Color>,

  /// badge icon. icon can be any Brand or Solid icons from fontawesome
  #[clap(long)]
  icon: Option<String>,

  /// icon color. Must be a valid css color
  #[clap(long)]
  icon_color: Option<Color>,

  /// output svg to file
  #[clap(short, long)]
  out: Option<PathBuf>,

  /// badge content
  #[clap()]
  content: Content,
}

fn main() -> Result<(), Box<dyn Error>> {
  let opt = Opt::parse();

  if matches!(&opt.icon, Some(icon) if !icon_exists(icon)) {
    return Err("Icon does not exists. Try using a fontawesome icon name".into());
  }

  let mut badge = Badge::new();

  if let Some(sub) = &opt.subject {
    badge.subject(sub);
  }
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

  let svg = match opt.content {
    Content::Data(d) => badge.data(&d.0).to_string(),
    Content::Text(t) => badge.text(&t).to_string(),
  };

  if let Some(out_file) = opt.out {
    let mut file = File::create(&out_file).unwrap();
    file.write_all(svg.as_bytes()).unwrap();
  } else {
    println!("{}", svg);
  }
  Ok(())
}
