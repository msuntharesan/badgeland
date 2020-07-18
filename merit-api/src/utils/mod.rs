pub mod error;

use merit::{Size, Styles, BadgeData};
use serde::Deserialize;
use std::str;

#[derive(Deserialize, Debug)]
pub struct QueryInfo {
  pub color: Option<String>,
  pub icon: Option<String>,
  pub icon_color: Option<String>,
  pub style: Option<Styles>,
  pub size: Option<Size>,
}

#[derive(Deserialize)]
pub struct BadgeOptions {
  pub text: Option<String>,
  pub subject: String,
  pub style: Option<Styles>,
  pub size: Option<Size>,
  pub color: Option<String>,
  pub icon: Option<String>,
  pub icon_color: Option<String>,
  pub data: Option<BadgeData>,
}
