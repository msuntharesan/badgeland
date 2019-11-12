use crate::utils::{
  error::{BadgeError, BadgeErrorBuilder},
  merit_query::{create_badge, BadgeSize, QueryInfo},
};
use actix_web::{http::StatusCode, web, HttpResponse};
use chrono::prelude::*;
use futures::Future;
use humanize::*;
use itertools::Itertools;
use merit::{Badge, Icon, Size, Styles};
use reqwest::r#async as req;
use serde_derive::Deserialize;
use serde_json::Value;
use std::{error::Error as StdError, str};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .data(req::Client::new())
    .service(
      web::scope("/npm/@{scope}/{package}")
        .route("/lic", web::get().to_async(npm_license_handler))
        .route("/dl/{period}", web::get().to_async(npm_dl_numbers))
        .route("/hist/{period}", web::get().to_async(npm_historical_chart))
        .route("/{tag}", web::get().to_async(npm_v_handler))
        .route("/", web::get().to_async(npm_v_handler))
        .route("", web::get().to_async(npm_v_handler)),
    )
    .service(
      web::scope("/npm/{package}")
        .route("/lic", web::get().to_async(npm_license_handler))
        .route("/dl/{period}", web::get().to_async(npm_dl_numbers))
        .route("/hist/{period}", web::get().to_async(npm_historical_chart))
        .route("/{tag}", web::get().to_async(npm_v_handler))
        .route("/", web::get().to_async(npm_v_handler))
        .route("", web::get().to_async(npm_v_handler)),
    );
}

const UNPKG_API_PATH: &'static str = "https://unpkg.com";
const DOWNLOAD_COUNT_PATH: &'static str = "https://api.npmjs.org/downloads/";

#[derive(Debug, Deserialize, Clone)]
enum Period {
  #[serde(alias = "d")]
  Daily,
  #[serde(alias = "w")]
  Weekly,
  #[serde(alias = "m")]
  Monthly,
  #[serde(alias = "y")]
  Yearly,
}

#[derive(Deserialize, Debug, Clone)]
struct NPMParams {
  scope: Option<String>,
  package: String,
  tag: Option<String>,
  period: Option<Period>,
}

impl NPMParams {
  fn to_path(self: &Self, api_path: &str) -> String {
    match (&self.scope, &self.tag) {
      (Some(s), Some(t)) => format!(
        "{api_path}/@{scope}/{package}@{tag}/package.json",
        api_path = api_path,
        scope = s,
        package = self.package,
        tag = t
      ),
      (Some(s), _) => format!(
        "{api_path}/@{scope}/{package}/package.json",
        api_path = api_path,
        scope = s,
        package = self.package
      ),
      (_, Some(t)) => format!(
        "{api_path}/{package}@{tag}/package.json",
        api_path = api_path,
        package = self.package,
        tag = t
      ),
      (_, _) => format!(
        "{api_path}/{package}/package.json",
        api_path = api_path,
        package = self.package
      ),
    }
  }
  fn to_dl_path(self: &Self, api_path: &str, is_range: bool) -> String {
    let mut path_str = format!("{}{}/", api_path, if is_range { "range" } else { "point" });

    match (&self.period, is_range) {
      (Some(Period::Daily), false) => {
        path_str.push_str("last-day/");
      }
      (Some(Period::Weekly), false) => {
        path_str.push_str("last-week/");
      }
      (Some(Period::Monthly), false) => {
        path_str.push_str("last-month/");
      }
      (Some(Period::Yearly), false) => {
        path_str.push_str("last-year/");
      }
      (Some(Period::Daily), true) => {
        path_str.push_str("last-week/");
      }
      (Some(Period::Weekly), true) => {
        path_str.push_str("last-month/");
      }
      (Some(Period::Monthly), true) => {
        path_str.push_str("last-year/");
      }
      (Some(Period::Yearly), true) => {
        let end = Utc::today();
        let start_year = &end.year() - 10;
        let start = Utc.ymd(start_year, 1, 1);
        let range = format!("{}:{}/", start.format("%Y-%m-%d"), end.format("%Y-%m-%d"));
        path_str.push_str(&range);
      }
      _ => {}
    };

    if let Some(s) = &self.scope {
      path_str.push_str(&format!("@{}/", s));
    }
    path_str.push_str(&self.package);
    path_str
  }
}

fn npm_get(client: &req::Client, path_str: &str) -> impl Future<Item = Value, Error = BadgeError> {
  client
    .get(path_str)
    .header("accept", "application/json")
    .send()
    .and_then(|mut resp: req::Response| resp.json::<Value>())
    .map_err(|err: reqwest::Error| {
      BadgeErrorBuilder::new()
        .description(err.description())
        .service("npm")
        .status(err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .url(err.url().map(|url: &reqwest::Url| url.as_str()))
        .build()
    })
}

fn npm_license_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<NPMParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = BadgeError> {
  let path = params.to_path(UNPKG_API_PATH);

  npm_get(&client, &path)
    .and_then(|value: Value| {
      value
        .get("license")
        .and_then(|v: &Value| v.as_str().map(String::from))
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'license' in {:?}", value))
            .service("npm")
            .build(),
        )
    })
    .and_then(move |license: String| {
      let badge = create_badge("licence", &license, None, &query);

      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn npm_dl_numbers(
  client: web::Data<req::Client>,
  (params, query): (web::Path<NPMParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = BadgeError> {
  let path = params.to_dl_path(&DOWNLOAD_COUNT_PATH, false);

  let opt = HumanizeOptions::builder()
    .lower_case(true)
    .precision(1usize)
    .build()
    .unwrap();

  npm_get(&client, &path)
    .and_then(|value: Value| {
      value
        .get("downloads")
        .and_then(|v: &Value| v.as_f64())
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::NOT_FOUND)
            .description(format!("Cannot find property 'downloads' in {:?}", value))
            .service("npm")
            .build(),
        )
    })
    .and_then(move |v: f64| {
      let subject = match &params.period {
        Some(Period::Daily) => "last-day",
        Some(Period::Weekly) => "last-week",
        Some(Period::Monthly) => "last-month",
        Some(Period::Yearly) => " last-year",
        _ => "",
      };
      let text = v.humanize(opt).unwrap();
      let mut badge = create_badge(&subject, &text, Some("#8254ed"), &query);
      let icon = Icon::new("download");
      badge.icon(icon.build());
      if let Some(bs) = &query.size {
        badge.size(match bs {
          BadgeSize::Large => Size::Large,
          BadgeSize::Medium => Size::Medium,
          BadgeSize::Small => Size::Small,
        });
      }
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn npm_historical_chart(
  client: web::Data<req::Client>,
  (params, query): (web::Path<NPMParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = BadgeError> {
  let path = &params.to_dl_path(DOWNLOAD_COUNT_PATH, true);

  npm_get(&client, &path)
    .and_then(|value: Value| -> Result<Vec<Value>, BadgeError> {
      value
        .get("downloads")
        .and_then(|v: &Value| v.as_array().cloned())
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .description(format!("Cannot find property 'downloads' in {:?}", &value))
            .service("npm")
            .build(),
        )
    })
    .and_then(|dls: Vec<Value>| {
      dls
        .iter()
        .map(|dl: &Value| match (dl.get("day"), dl.get("downloads")) {
          (Some(d), Some(c)) => Some((
            d.as_str().map(String::from).unwrap(),
            c.as_i64().clone().unwrap(),
          )),
          _ => None,
        })
        .collect::<Option<Vec<(String, i64)>>>()
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .description(format!("Cannot find property 'downloads' in {:?}", dls))
            .service("npm")
            .build(),
        )
    })
    .and_then(move |dls: Vec<((String, i64))>| {
      let dls = dls
        .iter()
        .group_by(|(day, _)| {
          let date = NaiveDate::parse_from_str(day, "%F").unwrap();
          match &params.period {
            Some(Period::Daily) => date.format("%F").to_string(),
            Some(Period::Weekly) => date.format("%Y-%U").to_string(),
            Some(Period::Monthly) => date.format("%Y-%m").to_string(),
            Some(Period::Yearly) => date.format("%Y-%m").to_string(),
            _ => "".to_string(),
          }
        })
        .into_iter()
        .map(|(_, group)| group.map(|(_, dls)| dls).sum::<i64>())
        .collect::<Vec<i64>>();

      let subject = match &params.period {
        Some(Period::Daily) => "Daily",
        Some(Period::Weekly) => "Weekly",
        Some(Period::Monthly) => "Monthly",
        Some(Period::Yearly) => "Yearly",
        _ => "",
      };
      let mut badge = Badge::new(subject);
      badge.data(dls);
      badge.color("8254ed");
      badge.icon(Icon::new("download").build());

      match &query.style {
        Some(x) if x == "flat" => {
          badge.style(Styles::Flat);
        }
        _ => {}
      }

      if let Some(bs) = &query.size {
        badge.size(match bs {
          BadgeSize::Large => Size::Large,
          BadgeSize::Medium => Size::Medium,
          BadgeSize::Small => Size::Small,
        });
      }
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn npm_v_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<NPMParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = BadgeError> {
  let path = params.to_path(UNPKG_API_PATH);

  npm_get(&client, &path)
    .and_then(|value: Value| {
      value
        .get("version")
        .and_then(|v: &Value| v.as_str().map(String::from))
        .ok_or(
          BadgeErrorBuilder::new()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .description(format!("Cannot find property 'version' in {:?}", value))
            .service("npm")
            .build(),
        )
    })
    .and_then(move |version: String| {
      let subject = match &params.tag {
        Some(t) => format!("@{}", t),
        None => "latest".to_string(),
      };

      let version = format!("{}", version);

      let mut badge = create_badge(&subject, &version, None, &query);
      badge.icon(Icon::new("npm").build());
      let svg = badge.to_string();

      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}
