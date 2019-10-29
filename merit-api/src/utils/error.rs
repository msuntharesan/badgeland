use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use merit::{icon_exists, Badge, IconBuilder};
use std::fmt;

pub struct BadgeError {
  status: StatusCode,
  description: String,
  url: Option<String>,
  service: Option<String>,
}

pub struct BadgeErrorBuilder {
  status: Option<StatusCode>,
  description: String,
  url: Option<String>,
  service: Option<String>,
}

impl BadgeErrorBuilder {
  pub fn new() -> Self {
    BadgeErrorBuilder {
      status: Some(StatusCode::INTERNAL_SERVER_ERROR),
      description: "unknown err".into(),
      url: None,
      service: None,
    }
  }
  pub fn description<T>(&mut self, description: T) -> &mut Self
  where
    T: Into<String>,
  {
    self.description = description.into();
    self
  }
  pub fn status(&mut self, status: StatusCode) -> &mut Self {
    self.status = Some(status);
    self
  }
  pub fn url<T>(&mut self, url: Option<T>) -> &mut Self
  where
    T: Into<String>,
  {
    self.url = url.map(|u| u.into());
    self
  }
  pub fn service<T>(&mut self, service: T) -> &mut Self
  where
    T: Into<String>,
  {
    self.service = Some(service.into());
    self
  }
  pub fn build(&self) -> BadgeError {
    BadgeError {
      description: self.description.to_owned(),
      status: self.status.unwrap(),
      url: self.url.to_owned(),
      service: self.service.to_owned(),
    }
  }
}

impl ResponseError for BadgeError {
  fn render_response(&self) -> HttpResponse {
    let mut err_badge = Badge::new("Error");

    match &self.service {
      Some(icon) if icon_exists(&icon) => {
        let icon = IconBuilder::new(&icon).build().unwrap();
        err_badge.icon(Some(icon));
      }
      Some(service) => {
        err_badge = Badge::new(&service);
      }
      _ => {}
    }
    err_badge.color("red");
    let text = u16::from(self.status).to_string();
    err_badge.text(&text);

    HttpResponse::InternalServerError()
      .status(self.status)
      .content_type("image/svg+xml")
      .body(err_badge.to_string())
  }
}

impl fmt::Display for BadgeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "URL: {:?} | Status Code: {:#?} | Reason: {}",
      self.url, self.status, self.description
    )
  }
}

impl fmt::Debug for BadgeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "URL: {:?} Status Code: {:#?} Reason: {}",
      self.url, self.status, self.description
    )
  }
}
