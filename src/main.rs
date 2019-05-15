// #![feature(proc_macro_hygiene)]

extern crate clap;
extern crate structopt;

use badge_maker::{icon_exists, Badge, Size, Styles};
use clap::arg_enum;
use std::{fs::File, io::prelude::*, path::PathBuf};
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

#[derive(StructOpt, Debug)]
#[structopt(name = "svg")]
struct Opt {
  #[structopt(long)]
  text: String,
  #[structopt(long)]
  subject: Option<String>,
  #[structopt(
    raw(possible_values = "&BadgeStyle::variants()", case_insensitive = "true"),
    long
  )]
  style: Option<BadgeStyle>,
  #[structopt(
    raw(possible_values = "&IconSize::variants()", case_insensitive = "true"),
    long
  )]
  size: Option<IconSize>,
  #[structopt(long)]
  color: Option<String>,
  #[structopt(long)]
  icon: Option<String>,
  #[structopt(long)]
  icon_colour: Option<String>,
  #[structopt(long, parse(from_os_str))]
  out: Option<PathBuf>,
}

fn main() {
  let opt: Opt = Opt::from_args();
  if let Some(icon) = &opt.icon {
    if !icon_exists(&icon) {
      eprintln!("Icon does not exists. Try using a fontawesome icon name");
      std::process::exit(1);
    }
  }
  let mut badge = Badge::new(&opt.text);
  if let Some(col) = opt.color {
    badge.color(col);
  }
  if let Some(subject) = opt.subject.as_ref() {
    badge.subject(subject);
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
  let badge = &badge.to_svg();

  if let Some(out_file) = opt.out {
    let mut file = File::create(&out_file).unwrap();
    file.write_all(badge.as_bytes()).unwrap();
  } else {
    println!("{}", badge.to_string());
  }
}
