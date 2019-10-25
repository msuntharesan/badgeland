use crate::utils::{
  error::MeritError,
  merit_query::{create_badge, QueryInfo},
};
use actix_web::{error, web, Error as ActixError, HttpResponse};
use chrono::prelude::*;
use futures::Future;
use humanize::*;
use itertools::Itertools;
use merit::IconBuilder;
use reqwest::r#async as req;
use serde_derive::Deserialize;
use serde_json::Value;
use std::str;

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

fn get_crate(
  client: &req::Client,
  path_str: &str,
) -> impl Future<Item = Value, Error = ActixError> {
  client
    .get(path_str)
    .header("accept", "application/json")
    .send()
    .and_then(|mut resp: req::Response| resp.json::<Value>())
    .map_err(MeritError::from)
    .map_err(ActixError::from)
}

fn crate_v_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<CrateParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let path = params.to_path(CRATES_API_PATH, None);
  get_crate(&client, &path).and_then(move |json: Value| {
    json
      .pointer("/crate/max_version")
      .and_then(|v: &Value| v.as_str().map(String::from))
      .ok_or(error::ErrorNotFound("Cannot find property"))
      .and_then(|ver| {
        let ver = format!("v{}", ver);
        let badge = create_badge("crates.io", &ver, Some("#e67233"), &query);

        let svg = badge.to_string();
        Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
      })
  })
}

fn crate_license_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<CrateParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let path = params.to_path(CRATES_API_PATH, None);
  get_crate(&client, &path).and_then(move |json: Value| {
    json
      .get("versions")
      .and_then(|v: &Value| v.as_array().cloned())
      .ok_or(error::ErrorNotFound("Cannot find property"))
      .and_then(|versions: Vec<Value>| {
        versions
          .into_iter()
          .find(|v: &Value| {
            !v.get("yanked")
              .and_then(|v: &Value| v.as_bool())
              .unwrap_or(false)
          })
          .ok_or(error::ErrorInternalServerError("Failed to parse yanked"))
          .and_then(|version: Value| {
            version
              .get("license")
              .and_then(|v: &Value| v.as_str().map(String::from))
              .ok_or(error::ErrorInternalServerError("Failed to parse license"))
          })
          .and_then(|v: String| {
            let badge = create_badge("license", &v, None, &query);
            let svg = badge.to_string();
            Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
          })
      })
  })
}

fn crate_dl_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<CrateParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let path = params.to_path(CRATES_API_PATH, None);

  let mut opts = humanize_options();
  opts.set_lowercase(true).set_precision(1);

  get_crate(&client, &path).and_then(move |json: Value| {
    json
      .pointer("/crate/downloads")
      .and_then(|v: &Value| v.as_i64().clone())
      .ok_or(error::ErrorNotFound("Cannot find property"))
      .and_then(|dls: i64| {
        let dls = dls.humanize(&opts).unwrap();
        let mut badge = create_badge("all-time", &dls, Some("#e67233"), &query);

        let icon = IconBuilder::new("download");
        badge.icon(icon.build());

        let svg = badge.to_string();
        Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
      })
  })
}

fn cargo_hist_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<CrateParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let path = params.to_path(CRATES_API_PATH, Some("downloads"));
  get_crate(&client, &path)
    .and_then(|value: Value| -> Result<Vec<Value>, ActixError> {
      value
        .get("version_downloads")
        .and_then(|v: &Value| v.as_array().cloned())
        .ok_or(error::ErrorInternalServerError(format!(
          "Failed to parse {:?}",
          &value
        )))
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
        .ok_or(error::ErrorInternalServerError(format!(
          "Failed to parse {:?}",
          &dls
        )))
    })
    .and_then(
      |dls: Vec<((String, i64))>| -> Result<Vec<i64>, ActixError> {
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
      },
    )
    .and_then(move |dls: Vec<i64>| {
      let mut badge = create_badge("last 90 days", "", Some("#e67233"), &query);
      badge.data(dls);
      badge.icon(IconBuilder::new("download").build());
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.data(req::Client::new()).service(
    web::scope("/crates/{package}")
      .route("/lic", web::get().to_async(crate_license_handler))
      .route("/dl", web::get().to_async(crate_dl_handler))
      .route("/hist", web::get().to_async(cargo_hist_handler))
      .route("/{tag}", web::get().to_async(crate_v_handler))
      .route("/", web::get().to_async(crate_v_handler))
      .route("", web::get().to_async(crate_v_handler)),
  );
}
