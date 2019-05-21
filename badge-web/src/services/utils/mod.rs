use actix_web::*;
use reqwest;
use std::fmt;

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

  // fn render_response(&self) -> HttpResponse {
  //   let mut resp = self.error_response();
  //   resp.headers_mut().insert(
  //     http::header::CONTENT_TYPE,
  //     http::header::HeaderValue::from_static("text/plain"),
  //   );
  //   resp
  // }
}

pub mod humanize;