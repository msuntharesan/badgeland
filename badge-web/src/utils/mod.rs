pub mod error;

pub mod badge_query {

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
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub style: Option<String>,
    pub size: Option<BadgeSize>,
  }
}
