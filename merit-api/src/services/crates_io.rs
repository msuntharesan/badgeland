use crate::utils::{
  error::{BadgeError, BadgeErrorBuilder},
  merit_query::{create_badge, QueryInfo},
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use chrono::prelude::*;
use futures::TryFutureExt;
use humanize::*;
use itertools::Itertools;
use merit::Icon;
use reqwest as req;
use serde_derive::Deserialize;
use serde_json::Value;
use std::{error::Error as StdError, str};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.data(req::Client::new()).service(
    web::scope("/crates/{package}")
      .route("/lic", web::get().to(crate_license_handler))
      .route("/dl", web::get().to(crate_dl_handler))
      .route("/hist", web::get().to(cargo_hist_handler))
      .route("/", web::get().to(crate_v_handler))
      .route("", web::get().to(crate_v_handler)),
  );
}

static CRATES_API_PATH: &'static str = "https://crates.io/api/v1/crates";

#[derive(Deserialize, Debug)]
struct CrateParams {
  package: String,
  tag: Option<String>,
}

impl CrateParams {
  fn to_path(self: &Self, api_path: &str, api: Option<&str>) -> String {
    let mut path = match &self.tag {
      Some(t) => format!(
        "{api_path}/{package}/{tag}",
        api_path = api_path,
        package = self.package,
        tag = t
      ),
      _ => format!(
        "{api_path}/{package}",
        api_path = api_path,
        package = self.package
      ),
    };

    if let Some(api) = api {
      let api = format!("/{}", api);
      path.push_str(&api);
    }
    path
  }
}

async fn get_crate(client: &req::Client, path_str: &str) -> Result<Value, BadgeError> {
  client
    .get(path_str)
    .header("accept", "application/json")
    .send()
    .and_then(|resp: req::Response| resp.json::<Value>())
    .await
    .map_err(|err: reqwest::Error| {
      BadgeErrorBuilder::new()
        .description(err.description())
        .service("crates.io")
        .status(err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .url(err.url().map(|url: &reqwest::Url| url.as_str()))
        .build()
    })
}

async fn crate_v_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<CrateParams>, web::Query<QueryInfo>),
) -> impl Responder {
  let path = params.to_path(CRATES_API_PATH, None);
  get_crate(&client, &path)
    .await
    .and_then(move |json: Value| {
      json
        .pointer("/crate/max_version")
        .and_then(|v: &Value| v.as_str().map(String::from))
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'version' in {:?}", json))
            .service("crates.io")
            .build(),
        )
        .and_then(|ver| {
          let ver = format!("v{}", ver);
          let badge = create_badge("crates.io", &ver, Some("#e67233"), &query);

          let svg = badge.to_string();
          Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
        })
    })
}

async fn crate_license_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<CrateParams>, web::Query<QueryInfo>),
) -> impl Responder {
  let path = params.to_path(CRATES_API_PATH, None);
  get_crate(&client, &path)
    .await
    .and_then(|json: Value| {
      json
        .pointer("/versions")
        .and_then(|v: &Value| v.as_array().cloned())
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'versions' in {:?}", json))
            .service("crates.io")
            .build(),
        )
    })
    .and_then(|versions: Vec<Value>| {
      versions
        .into_iter()
        .find(|v: &Value| {
          !v.get("yanked")
            .and_then(|v: &Value| v.as_bool())
            .unwrap_or(false)
        })
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'yanked' in 'versions'"))
            .service("crates.io")
            .build(),
        )
    })
    .and_then(|version: Value| {
      version
        .get("license")
        .and_then(|v: &Value| v.as_str().map(String::from))
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'license' in {:?}", version))
            .service("crates.io")
            .build(),
        )
    })
    .and_then(move |v: String| {
      let badge = create_badge("license", &v, None, &query);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

async fn crate_dl_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<CrateParams>, web::Query<QueryInfo>),
) -> impl Responder {
  let path = params.to_path(CRATES_API_PATH, None);

  let opts = HumanizeOptions::builder()
    .lower_case(true)
    .precision(1usize)
    .build()
    .unwrap();

  get_crate(&client, &path)
    .await
    .and_then(move |json: Value| {
      json
        .pointer("/crate/downloads")
        .and_then(|v: &Value| v.as_i64().clone())
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'downloads' in {:?}", json))
            .service("crates.io")
            .build(),
        )
        .and_then(|dls: i64| {
          let dls = dls.humanize(opts).unwrap();
          let mut badge = create_badge("all-time", &dls, Some("#e67233"), &query);

          let icon = Icon::new("download");
          badge.icon(icon.build());

          let svg = badge.to_string();
          Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
        })
    })
}

async fn cargo_hist_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<CrateParams>, web::Query<QueryInfo>),
) -> impl Responder {
  let path = params.to_path(CRATES_API_PATH, Some("downloads"));
  get_crate(&client, &path)
    .await
    .and_then(|value: Value| -> Result<Vec<Value>, BadgeError> {
      value
        .get("version_downloads")
        .and_then(|v: &Value| v.as_array().cloned())
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'downloads' in {:?}", value))
            .service("crates.io")
            .build(),
        )
    })
    .and_then(|dls: Vec<Value>| {
      dls
        .iter()
        .map(|dl: &Value| match (dl.get("date"), dl.get("downloads")) {
          (Some(d), Some(c)) => Some((
            d.as_str().map(String::from).unwrap(),
            c.as_i64().clone().unwrap(),
          )),
          _ => None,
        })
        .collect::<Option<Vec<(String, i64)>>>()
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'downloads' in {:?}", dls))
            .service("crates.io")
            .build(),
        )
    })
    .and_then(|dls: Vec<(String, i64)>| -> Result<Vec<i64>, BadgeError> {
      let dls = dls
        .iter()
        .group_by(|(day, _)| {
          let date = NaiveDate::parse_from_str(day, "%F").unwrap();
          date.format("%Y-%U").to_string()
        })
        .into_iter()
        .map(|(_, group)| group.map(|(_, dls)| dls).sum::<i64>())
        .collect::<Vec<i64>>();
      Ok(dls)
    })
    .and_then(move |dls: Vec<i64>| {
      let mut badge = create_badge("last 90 days", "", Some("#e67233"), &query);
      badge.data(dls);
      badge.icon(Icon::new("download").build());
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}
