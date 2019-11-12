use clap::arg_enum;
use merit::{icon_exists, Badge, BadgeData, Icon, Size, Styles};
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
    text: Option<String>,
    #[structopt(long)]
    subject: String,
    #[structopt(possible_values = &BadgeStyle::variants(), case_insensitive = true, long)]
    style: Option<BadgeStyle>,
    #[structopt(possible_values = &IconSize::variants(), case_insensitive = true, long)]
    size: Option<IconSize>,
    #[structopt(long)]
    /// rgb(), rgba(), 6 or 8 digit hex color or a valid css color name
    color: Option<String>,
    #[structopt(long)]
    /// Icon cany be any Brand or Solid icons from fontawesome
    icon: Option<String>,
    #[structopt(long)]
    /// rgb(), rgba(), 6 or 8 digit hex color or a valid css color name
    icon_colour: Option<String>,
    #[structopt(long, parse(from_os_str))]
    out: Option<PathBuf>,
    #[structopt(long, takes_value = true)]
    data: Option<BadgeData>,
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

    if let Some(col) = &opt.color {
        badge.color(col);
    }

    if let Some(t) = &opt.text {
        badge.text(t);
    }

    if let Some(icon) = &opt.icon {
        let mut i = Icon::new(icon);
        if let Some(ic) = opt.icon_colour {
            i.set_color(ic);
        }
        badge.icon(i.build());
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
