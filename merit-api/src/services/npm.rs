use crate::utils::{
  error::{BadgeError, BadgeErrorBuilder},
  merit_query::{create_badge, BadgeSize, QueryInfo},
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use chrono::prelude::*;
use chrono::Duration;
use futures::future::TryFutureExt;
use humanize::*;
use itertools::Itertools;
use merit::{Badge, Icon, Size, Styles};
use reqwest;
use serde_derive::Deserialize;
use serde_json::Value;
use std::{error::Error as StdError, str};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .data(reqwest::Client::new())
    .service(
      web::scope("/npm/@{scope}/{package}")
        .route("/lic", web::get().to(npm_license_handler))
        .route("/dl/{period}", web::get().to(npm_dl_numbers))
        .route("/hist/{period}", web::get().to(npm_historical_chart))
        .route("/{tag}", web::get().to(npm_v_handler))
        .route("/", web::get().to(npm_v_handler))
        .route("", web::get().to(npm_v_handler)),
    )
    .service(
      web::scope("/npm/{package}")
        .route("/lic", web::get().to(npm_license_handler))
        .route("/dl/{period}", web::get().to(npm_dl_numbers))
        .route("/hist/{period}", web::get().to(npm_historical_chart))
        .route("/{tag}", web::get().to(npm_v_handler))
        .route("/", web::get().to(npm_v_handler))
        .route("", web::get().to(npm_v_handler)),
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
  fn to_dl_point(&self, api_path: &str) -> String {
    let mut path_str = format!("{}point/", api_path);
    match &self.period {
      Some(Period::Daily) => {
        path_str.push_str("last-day/");
      }
      Some(Period::Weekly) => {
        path_str.push_str("last-week/");
      }
      Some(Period::Monthly) => {
        path_str.push_str("last-month/");
      }
      Some(Period::Yearly) => {
        path_str.push_str("last-year/");
      }
      _ => {}
    };
    if let Some(s) = &self.scope {
      path_str.push_str(&format!("@{}/", s));
    }
    path_str.push_str(&self.package);
    path_str
  }

  fn to_dl_range(&self, api_path: &str) -> String {
    let mut path_str = format!("{}range/", api_path);

    let end = Utc::today();

    let start = if let Some(Period::Yearly) = &self.period {
      Utc.ymd(&end.year() - 5, 1, 1)
    } else {
      *&end - Duration::days(365)
    };
    let range = format!("{}:{}/", start.format("%Y-%m-%d"), end.format("%Y-%m-%d"));
    path_str.push_str(&range);

    if let Some(s) = &self.scope {
      path_str.push_str(&format!("@{}/", s));
    }
    path_str.push_str(&self.package);
    path_str
  }
}

async fn npm_get(client: &reqwest::Client, path_str: &str) -> Result<Value, BadgeError> {
  client
    .get(path_str)
    .header("accept", "application/json")
    .send()
    .and_then(|resp: reqwest::Response| resp.json::<Value>())
    .await
    .map_err(|err: reqwest::Error| {
      BadgeErrorBuilder::new()
        .description(err.description())
        .service("npm")
        .status(err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .url(err.url().map(|url: &reqwest::Url| url.as_str()))
        .build()
    })
}

async fn npm_license_handler(
  client: web::Data<reqwest::Client>,
  (params, query): (web::Path<NPMParams>, web::Query<QueryInfo>),
) -> impl Responder {
  let path = params.to_path(UNPKG_API_PATH);

  npm_get(&client, &path)
    .await
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

async fn npm_dl_numbers(
  client: web::Data<reqwest::Client>,
  (params, query): (web::Path<NPMParams>, web::Query<QueryInfo>),
) -> impl Responder {
  let path = params.to_dl_point(&DOWNLOAD_COUNT_PATH);

  let opt = HumanizeOptions::builder()
    .lower_case(true)
    .precision(1usize)
    .build()
    .unwrap();

  npm_get(&client, &path)
    .await
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

async fn npm_historical_chart(
  client: web::Data<reqwest::Client>,
  (params, query): (web::Path<NPMParams>, web::Query<QueryInfo>),
) -> impl Responder {
  let path = &params.to_dl_range(DOWNLOAD_COUNT_PATH);
  println!("{}", path);
  npm_get(&client, &path)
    .await
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
    .and_then(move |dls: Vec<(String, i64)>| {
      let dls = dls
        .iter()
        .group_by(|(day, _)| {
          let date = NaiveDate::parse_from_str(day, "%F").unwrap();
          match &params.period {
            Some(Period::Daily) => date.format("%F").to_string(),
            Some(Period::Weekly) => date.format("%Y-%U").to_string(),
            Some(Period::Monthly) => date.format("%Y-%m").to_string(),
            Some(Period::Yearly) => date.format("%Y").to_string(),
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

async fn npm_v_handler(
  client: web::Data<reqwest::Client>,
  (params, query): (web::Path<NPMParams>, web::Query<QueryInfo>),
) -> impl Responder {
  let path = params.to_path(UNPKG_API_PATH);

  npm_get(&client, &path)
    .await
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
