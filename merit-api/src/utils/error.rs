use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use awc::error::{JsonPayloadError, SendRequestError};
use merit::{Badge, Icon};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BadgeError {
  #[error("HTTP Error: URL: {url:?} | Status Code: {status:#?} | Reason: {description:}")]
  Http {
    status: StatusCode,
    description: String,
    url: Option<String>,
  },
  #[error("Client Error: {0}")]
  Client(#[from] SendRequestError),
  #[error("Json Error: {0}")]
  ClientJsonError(#[from] JsonPayloadError),
}

impl Default for BadgeError {
  fn default() -> Self {
    BadgeError::Http {
      status: StatusCode::FORBIDDEN,
      description: "Forbidden".to_string(),
      url: None,
    }
  }
}

impl BadgeError {
  pub fn err_badge(&self) -> String {
    let icon = Icon::new("exclamation-circle").build().unwrap();
    let mut badge = Badge::new("Error");
    badge.icon(icon);
    badge.color("red");

    let text = match self {
      BadgeError::Http {
        status,
        description: _,
        url: _,
      } => status.as_u16().to_string(),
      BadgeError::Client(e) => match e {
        SendRequestError::Http(http_err) => http_err.status_code().as_u16().to_string(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16().to_string(),
      },
      _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16().to_string(),
    };
    badge.text(&text).to_string()
  }
}

impl ResponseError for BadgeError {
  fn error_response(&self) -> HttpResponse {
    let mut resp = HttpResponse::InternalServerError();
    println!("{:?}", self);
    resp.content_type("image/svg+xml");

    match self {
      BadgeError::Http {
        status,
        description: _,
        url: _,
      } => resp.status(*status).body(self.err_badge()),
      BadgeError::Client(e) => match e {
        SendRequestError::Http(http_err) => resp.status(http_err.status_code()).body(self.err_badge()),
        _ => resp.status(StatusCode::INTERNAL_SERVER_ERROR).body(self.err_badge()),
      },
      _ => resp.status(StatusCode::INTERNAL_SERVER_ERROR).body(self.err_badge()),
    }
  }
}
