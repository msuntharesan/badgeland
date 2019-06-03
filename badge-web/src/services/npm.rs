use super::utils::{
  badge_query::{BadgeSize, QueryInfo},
  humanize::*,
  ReqErr,
};
use actix_web::{http, web, Error as ActixError, FromRequest, HttpRequest, HttpResponse};
use badger::{Badge, Size, Styles};
use reqwest;
use serde_derive::Deserialize;
use serde_json::Value;
use std::{error::Error, str};

static UNPKGURL: &'static str = "https://unpkg.com";
static DOWNLOAD_COUNT: &'static str = "https://api.npmjs.org/downloads/";
// static NPMS: &'static str = "https://api.npms.io/v2/";

#[derive(Debug, Deserialize)]
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

#[derive(Deserialize, Debug)]
struct NPMVersion {
  scope: Option<String>,
  package: String,
  tag: Option<String>,
  period: Option<Period>,
}

#[derive(Debug, Deserialize)]
struct Unpkg {
  name: String,
  version: String,
}

fn npm_get(client: &reqwest::Client, path_str: &str, prop: &str) -> Result<Value, ReqErr> {
  client
    .get(path_str)
    .header("accept", "application/json")
    .send()
    .map_err(|err: reqwest::Error| ReqErr::new(err.status().unwrap(), err.description().to_owned()))
    .and_then(|mut resp: reqwest::Response| match resp.status() {
      http::StatusCode::OK => resp
        .json::<Value>()
        .map_err(|err: reqwest::Error| {
          ReqErr::new(err.status().unwrap(), err.description().to_owned())
        })
        .and_then(|mut j: Value| match j.get_mut(prop) {
          Some(v) => Ok(v.take()),
          None => Err(ReqErr::new(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            "Cannot find property".to_string(),
          )),
        }),
      _ => Err(ReqErr::new(
        resp.status(),
        "Cannot find package".to_string(),
      )),
    })
}

fn unpkg_path(params: &NPMVersion) -> String {
  match (&params.scope, &params.tag) {
    (Some(s), Some(t)) => format!(
      "{host}/@{scope}/{package}@{tag}/package.json",
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
      "{host}/{package}@{tag}/package.json",
      host = UNPKGURL,
      package = params.package,
      tag = t
    ),
    (_, _) => format!(
      "{host}/{package}/package.json",
      host = UNPKGURL,
      package = params.package
    ),
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
  match (&query.icon, &query.icon_color) {
    (Some(i), Some(c)) => {
      badge.icon(i, Some(c));
    }
    (Some(i), None) => {
      badge.icon(i, None);
    }
    (_, _) => (),
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

fn npm_v_handler(
  client: web::Data<reqwest::Client>,
  req: HttpRequest,
) -> Result<HttpResponse, ActixError> {
  let query = web::Query::<QueryInfo>::extract(&req).unwrap();
  let params = web::Path::<NPMVersion>::extract(&req).unwrap();
  let path = unpkg_path(&params);
  npm_get(&client, &path, "version")
    .and_then(|j: Value| {
      j.as_str().map(String::from).ok_or(ReqErr::new(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        "Cannot find property".to_string(),
      ))
    })
    .and_then(|version: String| {
      let subject = match &params.tag {
        Some(t) => format!("npm@{}", t),
        None => "npm".to_string(),
      };

      let version = format!("v{}", version);
      let badge = create_badge(&subject, &version, &query);
      let svg = badge.to_string();

      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
    .map_err(ActixError::from)
}

fn npm_license_handler(
  client: web::Data<reqwest::Client>,
  req: HttpRequest,
) -> Result<HttpResponse, ActixError> {
  let query = web::Query::<QueryInfo>::extract(&req).unwrap();
  let params = web::Path::<NPMVersion>::extract(&req).unwrap();
  let path = unpkg_path(&params);
  npm_get(&client, &path, "version")
    .and_then(|j: Value| {
      j.as_str().map(String::from).ok_or(ReqErr::new(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        "Cannot find property".to_string(),
      ))
    })
    .and_then(|license: String| {
      let badge = create_badge("license", &license, &query);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
    .map_err(ActixError::from)
}

fn npm_dl_path(params: &NPMVersion, is_range: bool) -> String {
  let mut path_str = format!(
    "{}{}/",
    DOWNLOAD_COUNT,
    if is_range { "range" } else { "point" }
  );
  match &params.period {
    Some(Period::Daily) if !is_range => {
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

  if let Some(s) = &params.scope {
    path_str.push_str(&format!("@{}/", s));
  }
  path_str.push_str(&params.package);
  path_str
}

fn npm_dl_numbers(
  client: web::Data<reqwest::Client>,
  req: HttpRequest,
) -> Result<HttpResponse, ActixError> {
  let mut opts = humanize_options();
  opts.set_lowercase(true);
  opts.set_precision(1);
  let params = web::Path::<NPMVersion>::extract(&req).unwrap();
  let path = npm_dl_path(&params, false);
  npm_get(&client, &path, "downloads")
    .and_then(|downloads: Value| {
      downloads.as_f64().ok_or(ReqErr::new(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to parse {:?}", downloads),
      ))
    })
    .and_then(|v: f64| {
      let mut badge = Badge::new("Downloads");
      let suffix = match &params.period {
        Some(Period::Daily) => "/d",
        Some(Period::Weekly) => "/w",
        Some(Period::Monthly) => "/m",
        Some(Period::Yearly) => "/y",
        _ => "",
      };
      let text = format!("{}{}", v.humanize(&opts).unwrap(), suffix);
      badge.text(text);
      badge.color("8254ed");
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
    .map_err(ActixError::from)
}

fn npm_historical_chart(
  client: web::Data<reqwest::Client>,
  req: HttpRequest,
) -> Result<HttpResponse, ActixError> {
  let mut opts = humanize_options();
  opts.set_lowercase(true);
  opts.set_precision(1);
  let params = web::Path::<NPMVersion>::extract(&req).unwrap();
  let path = npm_dl_path(&params, true);
  npm_get(&client, &path, "downloads")
    .and_then(|downloads: Value| -> Result<Vec<Value>, ReqErr> {
      downloads.as_array().cloned().ok_or(ReqErr::new(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to parse {:?}", &downloads),
      ))
    })
    .and_then(|ds: Vec<Value>| -> Result<Vec<i64>, ReqErr> {
      ds.iter()
        .map(|d: &Value| -> Result<i64, ReqErr> {
          d.get("downloads")
            .and_then(|d: &Value| d.as_i64())
            .ok_or(ReqErr::new(
              reqwest::StatusCode::INTERNAL_SERVER_ERROR,
              format!("Failed to parse {:?}", d),
            ))
        })
        .collect::<Result<Vec<i64>, ReqErr>>()
    })
    .and_then(|map: Vec<i64>| {
      let mut badge = Badge::new("Downloads");
      badge.data(map);
      badge.color("8254ed");
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
    .map_err(ActixError::from)
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .data(reqwest::Client::new())
    .service(
      web::scope("/npm/@{scope}/{package}")
        .route("lic", web::get().to(npm_license_handler))
        .route("dl/{period}", web::get().to(npm_dl_numbers))
        .route("hist/{period}", web::get().to(npm_historical_chart))
        .route("/{tag}", web::get().to_async(npm_v_handler))
        .route("/", web::get().to_async(npm_v_handler))
        .route("", web::get().to_async(npm_v_handler)),
    )
    .service(
      web::scope("/npm/{package}")
        .route("lic", web::get().to(npm_license_handler))
        .route("dl/{period}", web::get().to(npm_dl_numbers))
        .route("hist/{period}", web::get().to(npm_historical_chart))
        .route("/{tag}", web::get().to_async(npm_v_handler))
        .route("/", web::get().to_async(npm_v_handler))
        .route("", web::get().to_async(npm_v_handler)),
    );
}
