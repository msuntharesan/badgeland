use merit::{icon_exists, Badge, BadgeData, Icon, Size, Styles};
use pico_args::Arguments;
use std::{error::Error, ffi::OsStr, fs::File, io::prelude::*, path::PathBuf};

struct Opt {
  text: Option<String>,
  subject: String,
  style: Option<Styles>,
  size: Option<Size>,
  color: Option<String>,
  icon: Option<String>,
  icon_colour: Option<String>,
  out: Option<PathBuf>,
  data: Option<BadgeData>,
}

fn parse_path(s: &OsStr) -> Result<PathBuf, &'static str> {
  Ok(s.into())
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut opt = Arguments::from_env();
  let opt = Opt {
    text: opt.opt_value_from_str("--text")?,
    subject: opt
      .opt_value_from_str("--subject")?
      .expect("--subject can not be empty"),
    style: opt.opt_value_from_str("--style")?,
    color: opt.opt_value_from_str("--color")?,
    size: opt.opt_value_from_str("--size")?,
    icon: opt.opt_value_from_str("--icon")?,
    icon_colour: opt.opt_value_from_str("--icon_colour")?,
    data: opt.opt_value_from_str("--data")?,
    out: opt.opt_value_from_os_str("--out", parse_path)?,
  };

  if let Some(icon) = &opt.icon {
    if !icon_exists(icon) {
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

  if let Some(d) = opt.data {
    badge.data(d.0.into());
  }
  if let Some(s) = opt.style {
    badge.style(s);
  }
  if let Some(s) = opt.size {
    badge.size(s);
  }

  if let Some(out_file) = opt.out {
    let mut file = File::create(&out_file).unwrap();
    file.write_all(badge.to_string().as_bytes()).unwrap();
  } else {
    println!("{}", badge);
  }
  Ok(())
}
