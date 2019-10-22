use actix_web::{dev, HttpResponse, ResponseError};
use reqwest;
use std::{error::Error as StdError, fmt};

pub struct CustomError {
  pub status: reqwest::StatusCode,
  pub description: String,
}

pub enum ReqwestError {
  ReqwestErr(reqwest::Error),
  CustomErr(CustomError),
}

impl ReqwestError {
  fn status(&self) -> reqwest::StatusCode {
    match self {
      ReqwestError::ReqwestErr(err) => err
        .status()
        .or(Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
        .unwrap(),
      ReqwestError::CustomErr(err) => err.status,
    }
  }
  fn description(&self) -> String {
    match self {
      ReqwestError::ReqwestErr(err) => err.description().to_string(),
      ReqwestError::CustomErr(err) => err.description.to_string(),
    }
  }
  fn url(&self) -> Option<&reqwest::Url> {
    match self {
      ReqwestError::ReqwestErr(err) => err.url(),
      _ => None,
    }
  }
}

impl ResponseError for ReqwestError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::new(self.status()).set_body(dev::Body::from(format!(
      "URL: {:?} Description: {}",
      self.url(),
      self.description()
    )))
  }
}

impl fmt::Display for ReqwestError {
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

impl fmt::Debug for ReqwestError {
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

impl From<reqwest::Error> for ReqwestError {
  fn from(err: reqwest::Error) -> Self {
    ReqwestError::ReqwestErr(err)
  }
}

impl From<String> for ReqwestError {
  fn from(description: String) -> Self {
    ReqwestError::CustomErr(CustomError {
      status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
      description,
    })
  }
}
