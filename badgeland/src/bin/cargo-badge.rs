/*!
# Cli

Install using `cargo install badgeland --features cli`

```sh
Usage: badge -s <subject> [--style <style>] [--size <size>] [--color <color>] [--icon <icon>] [--icon-color <icon-color>] [--out <out>] [-t <text>] [--data <data>]

Fast badge generator for any purpose

Options:
  -s, --subject     badge subject
  --style           badge style. [possible values: flat | f, classic | c]
  --size            badge size. [possible values: large | l, medium | m, small | s]
  --color           badge color. Must be a valid css color
  --icon            badge icon. icon can be any Brand or Solid icons from fontawesome
  --icon-color      icon color. Must be a valid css color
  --out             output svg to file
  -t, --text        badge text
  --data            data for badge chart.
  --help            display usage information
```

*/

use badgeland::{icon_exists, Badge, BadgeData, Color, Icon, Size, Style};
use clap::{ArgGroup, Clap};
use std::{convert::TryFrom, error::Error, fs::File, io::prelude::*, path::PathBuf, str::FromStr};

#[derive(Debug, PartialEq)]
enum Content {
    Text(String),
    Data(BadgeData),
}

impl FromStr for Content {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BadgeData::from_str(s)
            .map(|d| Content::Data(d))
            .or(Ok(Content::Text(s.to_string())))
    }
}

#[derive(Clap, Debug)]
#[clap(group = ArgGroup::new("style").required(false))]
struct StyleArg {
    /// Flat badge style
    #[clap(short, long, group = "style")]
    flat: bool,

    /// Classic badge style (Default)
    #[clap(short, long, group = "style")]
    classic: bool,
}

impl From<StyleArg> for Style {
    fn from(s: StyleArg) -> Self {
        match (s.flat, s.classic) {
            (true, _) => Self::Flat,
            _ => Self::Classic,
        }
    }
}

#[derive(Clap, Debug)]
#[clap(group = ArgGroup::new("size").required(false))]
struct SizeArg {
    /// Small badge size (Default)
    #[clap(short = 'x', long, group = "size")]
    small: bool,

    /// Medium badge size
    #[clap(short, long, group = "size")]
    medium: bool,

    /// Large badge size
    #[clap(short, long, group = "size")]
    large: bool,
}

impl From<SizeArg> for Size {
    fn from(s: SizeArg) -> Self {
        match (s.large, s.medium, s.small) {
            (true, _, _) => Self::Large,
            (_, true, _) => Self::Medium,
            _ => Self::Small,
        }
    }
}

/// Fast badge generator for any purpose
#[derive(Debug, Clap)]
struct Opt {
    /// Badge subject
    #[clap(short, long)]
    subject: Option<String>,

    #[clap(flatten)]
    style: StyleArg,

    #[clap(flatten)]
    size: SizeArg,

    /// Badge color. Must be a valid css color
    #[clap(long)]
    color: Option<Color>,

    /// Badge icon. icon can be any `Brand` or `Solid` icons from fontawesome
    #[clap(long)]
    icon: Option<String>,

    /// Icon color. Must be a valid css color
    #[clap(long)]
    icon_color: Option<Color>,

    /// Output svg to file
    #[clap(short, long)]
    out: Option<PathBuf>,

    /// Badge content
    #[clap()]
    content: Content,
}

#[derive(Debug, Clap)]
#[clap(name = "cargo-badge", bin_name = "cargo")]
enum CargoCmd {
    #[clap(name = "badge")]
    Badge(Opt),
}

fn main() -> Result<(), Box<dyn Error>> {
    let badge_cmd = CargoCmd::parse();

    let CargoCmd::Badge(opt) = badge_cmd;

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

    badge.style(opt.style.into());

    badge.size(opt.size.into());

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
