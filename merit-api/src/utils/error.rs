use actix_web::{dev, HttpResponse, ResponseError};
use merit::icon_exists;
use reqwest;
use std::{error::Error as StdError, fmt};

pub struct BadgeError {
  pub status: reqwest::StatusCode,
  pub description: String,
  pub icon: Option<String>,
}

pub enum MeritError {
  ReqwestErr(reqwest::Error),
  CustomErr(BadgeError),
}

impl MeritError {
  fn status(&self) -> reqwest::StatusCode {
    match self {
      MeritError::ReqwestErr(err) => err
        .status()
        .or(Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
        .unwrap(),
      MeritError::CustomErr(err) => err.status,
    }
  }
  fn description(&self) -> String {
    match self {
      MeritError::ReqwestErr(err) => err.description().to_string(),
      MeritError::CustomErr(err) => err.description.to_string(),
    }
  }
  fn url(&self) -> Option<&reqwest::Url> {
    match self {
      MeritError::ReqwestErr(err) => err.url(),
      _ => None,
    }
  }
}

impl ResponseError for MeritError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::new(self.status()).set_body(dev::Body::from(format!(
      "URL: {:?} Description: {}",
      self.url(),
      self.description()
    )))
  }
}

impl fmt::Display for MeritError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "URL: {:?} Status Code: {:#?} Reason: {}",
      self.url(),
      self.status(),
      self.description()
    )
  }
}

impl fmt::Debug for MeritError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "URL: {:?} Status Code: {:#?} Reason: {}",
      self.url(),
      self.status(),
      self.description()
    )
  }
}

impl From<reqwest::Error> for MeritError {
  fn from(err: reqwest::Error) -> Self {
    MeritError::ReqwestErr(err)
  }
}

impl From<String> for MeritError {
  fn from(description: String) -> Self {
    MeritError::CustomErr(BadgeError {
      status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
      description,
      icon: None,
    })
  }
}

pub fn badge_error_for_service(
  service: &str,
  description: &str,
  status: Option<reqwest::StatusCode>,
) -> MeritError {
  let badge_err = BadgeError {
    status: status
      .or(Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
      .unwrap(),
    description: description.into(),
    icon: if icon_exists(service) {
      Some(service.into())
    } else {
      None
    },
  };
  MeritError::CustomErr(badge_err)
}
