pub mod error;

pub mod badge_query {

  use badger::*;
  use serde_derive::Deserialize;
  use std::str;

  #[derive(Debug, Deserialize)]
  pub enum BadgeSize {
    #[serde(alias = "large")]
    Large,
    #[serde(alias = "medium")]
    Medium,
    #[serde(alias = "small")]
    Small,
  }

  #[derive(Deserialize, Debug)]
  pub struct QueryInfo {
    pub color: Option<String>,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub style: Option<String>,
    pub size: Option<BadgeSize>,
  }

  pub fn create_badge<'a>(
    subject: &'a str,
    text: &'a str,
    color: Option<&'a str>,
    query: &'a QueryInfo,
  ) -> Badge<'a> {
    let mut badge = Badge::new(subject);
    badge.text(text);
    match &query.style {
      Some(x) if x == "flat" => {
        badge.style(Styles::Flat);
      }
      _ => {}
    }

    if let Some(i) = query.icon.as_ref() {
      let mut icon = IconBuilder::new(i);
      if let Some(ic) = query.icon_color.as_ref() {
        icon.set_color(ic);
      }
      badge.icon(icon.build());
    }

    if let Some(bs) = &query.size {
      badge.size(match bs {
        BadgeSize::Large => Size::Large,
        BadgeSize::Medium => Size::Medium,
        BadgeSize::Small => Size::Small,
      });
    }
    if let Some(color) = color {
      badge.color(color);
    }

    if let Some(color) = &query.color {
      badge.color(color);
    }
    badge
  }
}
