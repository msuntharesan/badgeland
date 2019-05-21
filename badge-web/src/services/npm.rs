use super::utils::{humanize::*, ReqErr};
use actix_web::{http, web, Error as ActixError, FromRequest, HttpRequest, HttpResponse};
use badge_maker::Badge;
use reqwest;
use serde_derive::Deserialize;
use serde_json;
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

fn unpkg_get(
  client: &reqwest::Client,
  params: &NPMVersion,
  package_json_prop: &str,
) -> Result<String, ReqErr> {
  let path = match (&params.scope, &params.tag) {
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
  };
  client
    .get(&path)
    .header("accept", "application/json")
    .send()
    .map_err(|err: reqwest::Error| ReqErr::new(err.status().unwrap(), err.description().to_owned()))
    .and_then(|mut resp: reqwest::Response| match resp.status() {
      http::StatusCode::OK => resp
        .json::<serde_json::Value>()
        .map_err(|err: reqwest::Error| {
          ReqErr::new(err.status().unwrap(), err.description().to_owned())
        })
        .and_then(|j: serde_json::Value| match j.get(package_json_prop) {
          Some(v) => Ok(v.as_str().unwrap().to_owned()),
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

fn npm_v_handler(
  client: web::Data<reqwest::Client>,
  req: HttpRequest,
) -> Result<HttpResponse, ActixError> {
  let params = web::Path::<NPMVersion>::extract(&req).unwrap();
  unpkg_get(&client, &params, "version")
    .map_err(ActixError::from)
    .and_then(|version| {
      let subject = match &params.tag {
        Some(t) => format!("npm@{}", t),
        None => "npm".to_string(),
      };
      let mut badge = Badge::new(&subject);
      badge.text(version);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn npm_license_handler(
  client: web::Data<reqwest::Client>,
  req: HttpRequest,
) -> Result<HttpResponse, ActixError> {
  let params = web::Path::<NPMVersion>::extract(&req).unwrap();
  unpkg_get(&client, &params, "license")
    .map_err(ActixError::from)
    .and_then(|license| {
      let mut badge = Badge::new("license");
      badge.text(license);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn npm_dl(
  client: &reqwest::Client,
  params: &NPMVersion,
  range: bool,
) -> Result<serde_json::Value, ReqErr> {
  let mut path_str = format!(
    "{}{}/",
    DOWNLOAD_COUNT,
    if range { "range" } else { "point" }
  );
  match &params.period {
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

  if let Some(s) = &params.scope {
    path_str.push_str(&format!("@{}/", s));
  }
  path_str.push_str(&params.package);
  client
    .get(&path_str)
    .header("accept", "application/json")
    .send()
    .map_err(|err: reqwest::Error| ReqErr::new(err.status().unwrap(), err.description().to_owned()))
    .and_then(|mut resp: reqwest::Response| match resp.status() {
      http::StatusCode::OK => resp
        .json::<serde_json::Value>()
        .map_err(|err: reqwest::Error| {
          ReqErr::new(err.status().unwrap(), err.description().to_owned())
        })
        .and_then(|mut j: serde_json::Value| match j.get_mut("downloads") {
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

fn npm_dl_numbers(
  client: web::Data<reqwest::Client>,
  req: HttpRequest,
) -> Result<HttpResponse, ActixError> {
  let mut opts = humanize_options();
  opts.set_lowercase(true);
  opts.set_precision(1);
  let params = web::Path::<NPMVersion>::extract(&req).unwrap();
  npm_dl(&client, &params, false)
    .and_then(|downloads| {
      let mut badge = Badge::new("Downloads");
      let suffix = match &params.period {
        Some(Period::Daily) => "/day",
        Some(Period::Weekly) => "/week",
        Some(Period::Monthly) => "/month",
        Some(Period::Yearly) => "/year",
        _ => "",
      };
      match downloads.as_f64() {
        Some(v) => {
          let text = format!("{}{}", v.humanize(&opts).unwrap(), suffix);
          badge.text(text);
        }
        None => {
          return Err(ReqErr::new(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to parse {:?}", downloads),
          ));
        }
      };

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
  npm_dl(&client, &params, true)
    .and_then(|downloads: serde_json::Value| {
      let map: Result<Vec<i64>, ReqErr> = downloads
        .as_array()
        .ok_or(ReqErr::new(
          reqwest::StatusCode::INTERNAL_SERVER_ERROR,
          format!("Failed to parse {:?}", downloads),
        ))
        .and_then(|ds: &Vec<serde_json::Value>| -> Result<Vec<i64>, ReqErr> {
          ds.iter()
            .map(|d: &serde_json::Value| -> Result<i64, ReqErr> {
              match d.get("downloads") {
                Some(d) if d.is_i64() => Ok(d.as_i64().unwrap()),
                _ => Err(ReqErr::new(
                  reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                  format!("Failed to parse {:?}", downloads),
                )),
              }
            })
            .collect::<Result<Vec<i64>, ReqErr>>()
        });
      let mut badge = Badge::new("Downloads");
      match &map {
        Ok(v) => {
          badge.data(v.clone());
        }
        _ => {}
      };
      badge.color("8254ed");
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
    .map_err(ActixError::from)

}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/npm")
      .data(reqwest::Client::new())
      .route("/l/@{scope}/{package}", web::get().to(npm_license_handler))
      .route("/l/{package}", web::get().to(npm_license_handler))
      .route(
        "/dl/{period}/@{scope}/{package}",
        web::get().to(npm_dl_numbers),
      )
      .route("/dl/{period}/{package}", web::get().to(npm_dl_numbers))
      .route(
        "/h/{period}/@{scope}/{package}",
        web::get().to(npm_historical_chart),
      )
      .route("/h/{period}/{package}", web::get().to(npm_historical_chart))
      .route("/@{scope}/{package}", web::get().to_async(npm_v_handler))
      .route(
        "/@{scope}/{package}/{tag}",
        web::get().to_async(npm_v_handler),
      )
      .route("/{package}", web::get().to_async(npm_v_handler))
      .route("/{package}/{tag}", web::get().to_async(npm_v_handler)),
  );
}
