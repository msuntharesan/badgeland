// #![feature(proc_macro_hygiene)]

use badge_maker::{icon_exists, Badge, Size, Styles};
use clap::arg_enum;
use std::{fs::File, io::prelude::*, num::ParseIntError, path::PathBuf, str::FromStr};
use structopt::StructOpt;

arg_enum! {
  #[derive(Debug)]
  enum IconSize {
    Large,
    Medium,
    Small
  }
}
arg_enum! {
  #[derive(Debug)]
  enum BadgeStyle {
    Flat,
    Classic
  }
}

#[derive(Debug)]
struct SparkData(Vec<i32>);

impl FromStr for SparkData {
  type Err = ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let values = s
      .split(",")
      .filter_map(|s| s.parse::<i32>().ok())
      // .filter (|x| x.is_ok())
      // .map(|x| x.unwrap())
      .collect::<Vec<_>>();
    Ok(SparkData(values))
  }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "svg")]
struct Opt {
  #[structopt(long)]
  text: Option<String>,
  #[structopt(long)]
  subject: String,
  #[structopt(raw(possible_values = "&BadgeStyle::variants()", case_insensitive = "true"), long)]
  style: Option<BadgeStyle>,
  #[structopt(raw(possible_values = "&IconSize::variants()", case_insensitive = "true"), long)]
  size: Option<IconSize>,
  #[structopt(long)]
  color: Option<String>,
  #[structopt(long)]
  icon: Option<String>,
  #[structopt(long)]
  icon_colour: Option<String>,
  #[structopt(long, parse(from_os_str))]
  out: Option<PathBuf>,
  #[structopt(long, raw(takes_value = "true"))]
  data: Option<SparkData>,
}

fn main() {
  let opt: Opt = Opt::from_args();
  if let Some(icon) = &opt.icon {
    if !icon_exists(&icon) {
      eprintln!("Icon does not exists. Try using a fontawesome icon name");
      std::process::exit(1);
    }
  }
  let mut badge = Badge::new(&opt.subject);
  if let Some(col) = opt.color {
    badge.color(col);
  }
  if let Some(t) = opt.text.as_ref() {
    badge.text(t);
  }
  match (opt.icon.as_ref(), opt.icon_colour.as_ref()) {
    (Some(icon), Some(color)) => {
      badge.icon(icon, Some(color));
    }
    (Some(icon), None) => {
      badge.icon(icon, None);
    }
    _ => {}
  }
  if let Some(d) = &opt.data {
    badge.data(d.0.to_owned());
  }

  badge
    .style(match opt.style.unwrap_or(BadgeStyle::Classic) {
      BadgeStyle::Flat => Styles::Flat,
      BadgeStyle::Classic => Styles::Classic,
    })
    .size(match opt.size.unwrap_or(IconSize::Small) {
      IconSize::Large => Size::Large,
      IconSize::Medium => Size::Medium,
      IconSize::Small => Size::Small,
    });

  if let Some(out_file) = opt.out {
    let mut file = File::create(&out_file).unwrap();
    file.write_all(badge.to_string().as_bytes()).unwrap();
  } else {
    println!("{}", badge);
  }
}
