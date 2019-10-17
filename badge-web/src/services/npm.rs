use crate::utils::{
  badge_query::{BadgeSize, QueryInfo},
  ReqErr,
};
use actix_web::{error, http, web, Error as ActixError, HttpResponse};
use badger::{Badge, IconBuilder, Size, Styles};
use chrono::prelude::*;
use futures::Future;
use humanize::*;
use itertools::Itertools;
use reqwest::r#async as req;
use serde_derive::Deserialize;
use serde_json::Value;
use std::str;

static UNPKGURL: &'static str = "https://unpkg.com";
static DOWNLOAD_COUNT: &'static str = "https://api.npmjs.org/downloads/";

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
  #[serde(alias = "t")]
  Total,
}

#[derive(Deserialize, Debug, Clone)]
struct NPMVersion {
  scope: Option<String>,
  package: String,
  tag: Option<String>,
  period: Option<Period>,
}

impl NPMVersion {
  fn to_path(self: &Self, host: &str) -> String {
    match (&self.scope, &self.tag) {
      (Some(s), Some(t)) => format!(
        "{host}/@{scope}/{package}@{tag}/package.json",
        host = host,
        scope = s,
        package = self.package,
        tag = t
      ),
      (Some(s), _) => format!(
        "{host}/@{scope}/{package}/package.json",
        host = host,
        scope = s,
        package = self.package
      ),
      (_, Some(t)) => format!(
        "{host}/{package}@{tag}/package.json",
        host = host,
        package = self.package,
        tag = t
      ),
      (_, _) => format!(
        "{host}/{package}/package.json",
        host = host,
        package = self.package
      ),
    }
  }
  fn to_dl_path(self: &Self, host: &str, is_range: bool) -> String {
    let mut path_str = format!("{}{}/", host, if is_range { "range" } else { "point" });

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

fn create_badge<'a>(subject: &'a str, text: &'a str, query: &'a QueryInfo) -> Badge<'a> {
  let mut badge = Badge::new(subject);
  badge.text(text);
  match &query.style {
    Some(x) if x == "flat" => {
      badge.style(Styles::Flat);
    }
    _ => {}
  }

  if let Some(i) = query.icon.as_ref() {
    let mut icon = IconBuilder::new(i);
    if let Some(ic) = query.icon_color.as_ref() {
      icon.set_color(ic);
    }
    badge.icon(icon.build());
  }

  if let Some(bs) = &query.size {
    badge.size(match bs {
      BadgeSize::Large => Size::Large,
      BadgeSize::Medium => Size::Medium,
      BadgeSize::Small => Size::Small,
    });
  }
  badge
}

fn npm_get(client: &req::Client, path_str: &str) -> impl Future<Item = Value, Error = ReqErr> {
  client
    .get(path_str)
    .header("accept", "application/json")
    .send()
    .map_err(ReqErr::from)
    .and_then(|mut resp: req::Response| match resp.status() {
      http::StatusCode::OK => Ok(resp.json::<Value>()),
      _ => Err(ReqErr::new(
        resp.status(),
        "Cannot find package".to_string(),
      )),
    })
    .and_then(|json| json.map_err(ReqErr::from).and_then(|j: Value| Ok(j)))
}

fn npm_license_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<NPMVersion>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let path = params.to_path(UNPKGURL);
  npm_get(&client, &path)
    .map_err(ActixError::from)
    .and_then(|mut value: Value| {
      value
        .pointer_mut("/license")
        .map(Value::take)
        .ok_or(error::ErrorInternalServerError(
          "Cannot find property".to_string(),
        ))
        .and_then(|v: Value| {
          v.as_str()
            .map(String::from)
            .ok_or(error::ErrorInternalServerError(
              "Cannot read property".to_string(),
            ))
        })
    })
    .and_then(move |license: String| {
      let badge = create_badge("license", &license, &query);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
    .map_err(ActixError::from)
}

fn npm_dl_numbers(
  client: web::Data<req::Client>,
  (params, query): (web::Path<NPMVersion>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let mut opts = humanize_options();
  opts.set_lowercase(true);
  opts.set_precision(1);
  let path = params.to_dl_path(&DOWNLOAD_COUNT, false);
  npm_get(&client, &path)
    .map_err(ActixError::from)
    .and_then(|value: Value| {
      value
        .get("downloads")
        .and_then(|v: &Value| v.as_f64())
        .ok_or(error::ErrorInternalServerError(format!(
          "Failed to parse {:?}",
          value
        )))
    })
    .and_then(move |v: f64| {
      let subject = match &params.period {
        Some(Period::Daily) => "last-day",
        Some(Period::Weekly) => "last-week",
        Some(Period::Monthly) => "last-month",
        Some(Period::Yearly) => " last-year",
        _ => "",
      };

      let mut badge = Badge::new(subject);
      let text = v.humanize(&opts).unwrap();
      badge.text(text);
      badge.color("8254ed");
      let icon = IconBuilder::new("download");
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
    .map_err(ActixError::from)
}

fn npm_historical_chart(
  client: web::Data<req::Client>,
  (params, query): (web::Path<NPMVersion>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let mut opts = humanize_options();
  opts.set_lowercase(true);
  opts.set_precision(1);
  let path = params.to_dl_path(DOWNLOAD_COUNT, true);
  npm_get(&client, &path)
    .map_err(ActixError::from)
    .and_then(|value: Value| -> Result<Vec<Value>, ActixError> {
      value
        .get("downloads")
        .and_then(|v: &Value| v.as_array().cloned())
        .ok_or(error::ErrorInternalServerError(format!(
          "Failed to parse {:?}",
          &value
        )))
    })
    .and_then(move |ds: Vec<Value>| {
      ds.iter()
        .map(|d: &Value| -> Result<((String, i64)), ActixError> {
          match (d.get("day"), d.get("downloads")) {
            (Some(d), Some(c)) => Ok((
              d.as_str().map(String::from).unwrap(),
              c.as_i64().clone().unwrap(),
            )),
            _ => Err(error::ErrorInternalServerError(format!(
              "Failed to parse {:?}",
              &d
            ))),
          }
        })
        .collect::<Result<Vec<(String, i64)>, ActixError>>()
        .and_then(|c| {
          c.iter()
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
            .map(|(_, group)| Ok(group.map(|(_, dls)| dls).sum::<i64>()))
            .collect::<Result<Vec<i64>, ActixError>>()
        })
        .map_err(ActixError::from)
        .and_then(|dls| {
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
          badge.icon(IconBuilder::new("download").build());
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
    })
    .map_err(ActixError::from)
}

fn npm_v_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<NPMVersion>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let path = params.to_path(UNPKGURL);

  npm_get(&client, &path)
    .map_err(ActixError::from)
    .and_then(|value: Value| {
      value
        .get("version")
        .and_then(|v: &Value| v.as_str().map(String::from))
        .ok_or(error::ErrorInternalServerError(
          "Cannot read property".to_string(),
        ))
    })
    .and_then(move |version: String| {
      let subject = match &params.tag {
        Some(t) => format!("@{}", t),
        None => "latest".to_string(),
      };

      let version = format!("{}", version);
      let mut badge = create_badge(&subject, &version, &query);
      badge.icon(IconBuilder::new("npm").build());
      let svg = badge.to_string();

      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
    .map_err(ActixError::from)
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .data(req::Client::new())
    .service(
      web::scope("/npm/@{scope}/{package}")
        .route("lic", web::get().to_async(npm_license_handler))
        .route("dl/{period}", web::get().to_async(npm_dl_numbers))
        .route("hist/{period}", web::get().to_async(npm_historical_chart))
        .route("/{tag}", web::get().to_async(npm_v_handler))
        .route("/", web::get().to_async(npm_v_handler))
        .route("", web::get().to_async(npm_v_handler)),
    )
    .service(
      web::scope("/npm/{package}")
        .route("lic", web::get().to_async(npm_license_handler))
        .route("dl/{period}", web::get().to_async(npm_dl_numbers))
        .route("hist/{period}", web::get().to_async(npm_historical_chart))
        .route("/{tag}", web::get().to_async(npm_v_handler))
        .route("/", web::get().to_async(npm_v_handler))
        .route("", web::get().to_async(npm_v_handler)),
    );
}
