use badger::{icon_exists, Badge, IconBuilder, Size, Styles};
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
struct SparkData(Vec<i64>);

impl FromStr for SparkData {
  type Err = ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let values = s
      .split(",")
      .filter_map(|s| s.parse::<i64>().ok())
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
  #[structopt(possible_values = &BadgeStyle::variants(), case_insensitive = true, long)]
  style: Option<BadgeStyle>,
  #[structopt(possible_values = &IconSize::variants(), case_insensitive = true, long)]
  size: Option<IconSize>,
  #[structopt(long)]
  color: Option<String>,
  #[structopt(long)]
  icon: Option<String>,
  #[structopt(long)]
  icon_colour: Option<String>,
  #[structopt(long, parse(from_os_str))]
  out: Option<PathBuf>,
  #[structopt(long, takes_value = true)]
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
  if let Some(icon) = opt.icon.as_ref() {
    let mut i = IconBuilder::new(icon);
    if let Some(ic) = opt.icon_colour.as_ref() {
      i.set_color(ic);
    }
    badge.icon(i.build());
  }
  // let icon = match(opt.icon.as_ref(), opt.icon_colour.as_ref()){
  // (Some(i),Some(ic))=>Some(Icon::new(i).unwrap().color(ic).create()),
  // (Some(i), _)=>Icon::new(i).unwrap().create(),
  // _=> None
  // };
  // let icon = if let Some(icon) = opt.icon.as_ref() {
  //   Icon::new(icon)
  // } else {
  //   None
  // };
  // if let (Some(mut icon), Some(color))  = (icon, opt.icon_colour.as_ref()) {
  //   icon.color(color);
  // }

  // badge.icon(icon);

  // match (opt.icon.as_ref(), opt.icon_colour.as_ref()) {
  //   (Some(icon), Some(color)) => {
  //     Icon::new(icon).color(color);
  //   }
  //   (Some(icon), None) => {
  //     Icon::new(icon);
  //   }
  //   _ => None
  // };
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
