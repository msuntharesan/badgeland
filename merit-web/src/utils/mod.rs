pub mod error;

use merit::{BadgeData, Color, Size, Styles};
use serde::Deserialize;
use std::str;

#[derive(Deserialize, Debug)]
pub struct QueryInfo {
  pub source: Option<String>,
  pub color: Option<Color>,
  pub icon: Option<String>,
  pub icon_color: Option<Color>,
  pub style: Option<Styles>,
  pub size: Option<Size>,
}

#[derive(Deserialize)]
pub struct BadgeOptions {
  pub text: Option<String>,
  pub subject: String,
  pub style: Option<Styles>,
  pub size: Option<Size>,
  pub color: Option<Color>,
  pub icon: Option<String>,
  pub icon_color: Option<Color>,
  pub data: Option<BadgeData>,
}
