/*!
# Cli

Install using `cargo install badgeland --features cli`

```sh
Fast badge generator for any purpose

USAGE:
    cargo badge [OPTIONS] <CONTENT>

ARGS:
    <CONTENT>    Badge content. Can be string or csv

OPTIONS:
    -c, --classic                    Classic badge style (Default)
        --color <COLOR>              Badge color. Must be a valid css color
    -f, --flat                       Flat badge style
    -z  --social                     Social badge style
    -h, --help                       Print help information
        --icon <ICON>                Badge icon. Icons are from
                                     <https://fontawesome.com/search?s=brands>,
                                     <https://fontawesome.com/search?s=solid> and
                                     <https://simpleicons.org/>
        --icon-color <ICON_COLOR>    Icon color. Must be a valid css color
    -l, --large                      Large badge size
    -m, --medium                     Medium badge size
    -o, --out <OUT>                  Output svg to file
    -s, --subject <SUBJECT>          Badge subject
    -x, --small                      Small badge size (Default)
```

*/

use badgeland::{icon_exists, Badge, BadgeData, Color, Icon, Size, Style};
use clap::{ArgGroup, Parser};
use std::{convert::TryFrom, error::Error, fs::File, io::prelude::*, path::PathBuf, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("style").required(false))]
struct StyleArg {
    /// Flat badge style
    #[clap(short, long, group = "style", action)]
    flat: bool,

    /// Classic badge style (Default)
    #[clap(short, long, group = "style", action)]
    classic: bool,

    /// Social badge style
    #[clap(short = 'z', long, group = "style", action)]
    social: bool,
}

impl From<StyleArg> for Style {
    fn from(s: StyleArg) -> Self {
        if s.flat {
            Self::Flat
        } else if s.social {
            Self::Social
        } else {
            Self::Classic
        }
    }
}

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("size").required(false))]
struct SizeArg {
    /// Small badge size (Default)
    #[clap(short = 'x', long, group = "size", action)]
    small: bool,

    /// Medium badge size
    #[clap(short, long, group = "size", action)]
    medium: bool,

    /// Large badge size
    #[clap(short, long, group = "size", action)]
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
#[derive(Debug, Parser)]
struct Opt {
    /// Badge subject
    #[clap(short, long, value_parser)]
    subject: Option<String>,

    #[clap(flatten)]
    style: StyleArg,

    #[clap(flatten)]
    size: SizeArg,

    /// Badge color. Must be a valid css color
    #[clap(long, value_parser)]
    color: Option<Color>,

    /// Badge icon. Icons are from <https://fontawesome.com/search?s=brands>, <https://fontawesome.com/search?s=solid> and <https://simpleicons.org/>
    #[clap(long, value_parser)]
    icon: Option<String>,

    /// Icon color. Must be a valid css color
    #[clap(long, value_parser)]
    icon_color: Option<Color>,

    /// Output svg to file
    #[clap(short, long, value_parser)]
    out: Option<PathBuf>,

    /// Badge content. Can be string or csv
    #[clap(value_parser)]
    content: Content,
}

#[derive(Debug, Parser)]
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
        Content::Data(d) => badge.data(d.as_ref()).to_string(),
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
