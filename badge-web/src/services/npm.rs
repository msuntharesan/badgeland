use actix_web::{http, web, Error as ActixError, FromRequest, HttpRequest, HttpResponse};
use badge_maker::Badge;
use reqwest;
// use percent_encoding::percent_encode;
use super::utils::ReqErr;
use serde_derive::Deserialize;
use std::{error::Error, str};

static UNPKGURL: &'static str = "https://unpkg.com";

#[derive(Debug, Deserialize)]
enum Range {
  #[serde(alias = "d")]
  Daily,
  #[serde(alias = "w")]
  Weekly,
  #[serde(alias = "m")]
  Monthly,
  #[serde(alias = "y")]
  Yearly,
  #[serde(alias = "t")]
  Total,
}

#[derive(Deserialize)]
struct NPMVersion {
  scope: Option<String>,
  package: String,
  tag: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Unpkg {
  name: String,
  version: String,
}

fn npm_v_handler(client: web::Data<reqwest::Client>, req: HttpRequest) -> Result<HttpResponse, ActixError> {
  let params = web::Path::<NPMVersion>::extract(&req).unwrap();
  let path = match (&params.scope, &params.tag) {
    (Some(s), Some(t)) => format!(
      "{host}/@{scope}/{package}/{tag}/package.json",
      host = UNPKGURL,
      scope = s,
      package = params.package,
      tag = t
    ),
    (Some(s), _) => format!(
      "{host}/@{scope}/{package}/package.json",
      host = UNPKGURL,
      scope = s,
      package = params.package
    ),
    (_, Some(t)) => format!(
      "{host}/{package}/{tag}/package.json",
      host = UNPKGURL,
      package = params.package,
      tag = t
    ),
    (_, _) => format!(
      "{host}/{package}/package.json",
      host = UNPKGURL,
      package = params.package
    ),
  };
  client
    .get(&path)
    .header("accept", "application/json")
    .send()
    .map_err(|err: reqwest::Error| ActixError::from(ReqErr::new(err.status().unwrap(), err.description().to_owned())))
    .and_then(|mut resp: reqwest::Response| match resp.status() {
      http::StatusCode::OK => resp
        .json::<serde_json::Value>()
        .map_err(|err: reqwest::Error| {
          println!("{:?}", err.description());
          ActixError::from(ReqErr::new(err.status().unwrap(), err.description().to_owned()))
        })
        .and_then(|j| {
          let mut badge = Badge::new("npm");
          let version = j.get("version").unwrap().as_str().unwrap();
          badge.text(version);
          let svg = badge.to_string();
          Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
        }),
      _ => Err(ActixError::from(ReqErr::new(
        resp.status(),
        "Cannot find package".to_string(),
      ))),
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/npm")
      .data(reqwest::Client::new())
      .route("/@{scope}/{package}", web::get().to_async(npm_v_handler))
      .route("/{package}", web::get().to_async(npm_v_handler))
      .route("/@{scope}/{package}/{tag}", web::get().to_async(npm_v_handler))
      .route("/{package}/{tag}", web::get().to_async(npm_v_handler)),
  );
}
