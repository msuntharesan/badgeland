use crate::utils::{
  error::{BadgeError, BadgeErrorBuilder},
  merit_query::{create_badge, QueryInfo},
};
use actix_web::{http::StatusCode, web, HttpResponse};
use futures::Future;
use humanize::*;
use merit::Icon;
use reqwest::r#async as req;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, error::Error as StdErr, str};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.data(req::Client::new()).service(
    web::scope("/github/{owner}/{name}")
      .route("/lic", web::get().to_async(github_lic_handler))
      .route("/stars", web::get().to_async(github_stars_handler))
      .route("/watchers", web::get().to_async(github_watch_handler))
      .route("/forks", web::get().to_async(github_fork_handler))
      .route("/release/{tag_name}", web::get().to_async(not_impl)),
  );
}

const QUERY: &'static str = include_str!("../resx/github_query.graphql");

#[derive(Serialize)]
struct Variables {
  owner: String,
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryBody<'a, Variables>
where
  Variables: serde::Serialize,
{
  variables: Variables,
  query: &'a str,
  #[serde(rename = "operationName")]
  operation_name: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
struct GithubParams {
  owner: String,
  name: String,
  tag_name: Option<String>,
}

static GITHUB_GQL_ENDPOINT: &'static str = "https://api.github.com/graphql";

fn make_req<'a>(
  client: &req::Client,
  variables: Variables,
  operation_name: &'a str,
) -> impl Future<Item = Value, Error = BadgeError> {
  let query = QueryBody {
    variables,
    query: QUERY,
    operation_name,
  };
  client
    .post(GITHUB_GQL_ENDPOINT)
    .bearer_auth(env::var("GH_ACCESS_TOKEN").unwrap())
    .json(&query)
    .send()
    .and_then(|mut resp: req::Response| resp.json::<Value>())
    .map_err(|err: reqwest::Error| {
      BadgeErrorBuilder::new()
        .description(err.description())
        .service("github")
        .status(err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .url(err.url().map(|url: &reqwest::Url| url.as_str()))
        .build()
    })
}

fn github_lic_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<GithubParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = BadgeError> {
  let variables = Variables {
    owner: String::from(&params.owner),
    name: String::from(&params.name),
  };
  make_req(&client, variables, "GithubLicense")
    .and_then(|rd: Value| {
      let lic_str = rd
        .pointer("/data/repository/licenseInfo/spixId")
        .and_then(|lic: &Value| lic.as_str().map(String::from))
        .unwrap_or("no license".into());
      Ok(lic_str)
    })
    .and_then(move |lic_str: String| {
      let mut badge = create_badge("license", &lic_str, None, &query);
      let icon = Icon::new("github").build();
      badge.icon(icon);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn github_stars_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<GithubParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = BadgeError> {
  let variables = Variables {
    owner: String::from(&params.owner),
    name: String::from(&params.name),
  };

  let opt = HumanizeOptions::builder()
    .lower_case(true)
    .precision(1usize)
    .build()
    .unwrap();

  make_req(&client, variables, "GithubStarCount")
    .and_then(|rd: Value| {
      let star_count = rd
        .pointer("/data/repository/stargazers/totalCount")
        .and_then(|total: &Value| total.as_i64())
        .and_then(|v| v.humanize(opt))
        .unwrap_or("0".into());
      Ok(star_count)
    })
    .and_then(move |star_count| {
      let mut badge = create_badge("stars", &star_count, None, &query);
      let icon = Icon::new("github").build();
      badge.icon(icon);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn github_watch_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<GithubParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = BadgeError> {
  let variables = Variables {
    owner: String::from(&params.owner),
    name: String::from(&params.name),
  };

  let opt = HumanizeOptions::builder()
    .lower_case(true)
    .precision(1usize)
    .build()
    .unwrap();

  make_req(&client, variables, "GithubWatchCount")
    .and_then(|rd: Value| {
      let watchers = rd
        .pointer("/data/repository/watchers/totalCount")
        .and_then(|total: &Value| total.as_i64())
        .and_then(|v| v.humanize(opt))
        .unwrap_or("0".into());
      Ok(watchers)
    })
    .and_then(move |watchers| {
      let mut badge = create_badge("watchers", &watchers, None, &query);
      let icon = Icon::new("github").build();
      badge.icon(icon);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn github_fork_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<GithubParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = BadgeError> {
  let variables = Variables {
    owner: String::from(&params.owner),
    name: String::from(&params.name),
  };

  let opt = HumanizeOptions::builder()
    .lower_case(true)
    .precision(1usize)
    .build()
    .unwrap();

  make_req(&client, variables, "GithubForkCount")
    .and_then(|rd: Value| {
      let forks = rd
        .pointer("/data/repository/forkCount")
        .and_then(|total: &Value| total.as_i64())
        .and_then(|v| v.humanize(opt))
        .unwrap_or("0".into());
      Ok(forks)
    })
    .and_then(move |forks| {
      let count = forks.to_string();
      let mut badge = create_badge("forks", &count, None, &query);
      let icon = Icon::new("github").build();
      badge.icon(icon);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn not_impl() -> impl Future<Item = (), Error = BadgeError> {
  futures::done(Err(
    BadgeErrorBuilder::new()
      .service("github")
      .status(StatusCode::NOT_IMPLEMENTED)
      .build(),
  ))
}
