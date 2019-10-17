use actix_web::*;
use reqwest;
use std::{error::Error, fmt};

pub struct ReqErr {
  status: reqwest::StatusCode,
  reason: String,
}

impl ReqErr {
  pub fn new(status: reqwest::StatusCode, reason: String) -> Self {
    ReqErr { status, reason }
  }
}

impl fmt::Debug for ReqErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Status Code: {} Reason: {}", self.status, self.reason)
  }
}

impl fmt::Display for ReqErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Status Code: {} Reason: {}", self.status, self.reason)
  }
}

impl ResponseError for ReqErr {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::new(self.status).set_body(dev::Body::from(self.reason.to_owned()))
  }
}

impl From<reqwest::Error> for ReqErr {
  fn from(err: reqwest::Error) -> Self {
    match err.status() {
      Some(s) => ReqErr::new(s, err.description().to_owned()),
      _ => ReqErr::new(
        err
          .status()
          .or(Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
          .unwrap(),
        err.description().to_owned(),
      ),
    }
  }
}

pub mod humanize;

impl From<Box<dyn Error>> for ReqErr {
  fn from(err: Box<dyn Error>) -> Self {
    ReqErr::new(
      reqwest::StatusCode::INTERNAL_SERVER_ERROR,
      err.description().to_owned(),
    )
  }
}

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
