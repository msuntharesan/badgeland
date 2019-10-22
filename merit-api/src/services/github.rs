use crate::utils::{
  merit_query::{create_badge, QueryInfo},
  error::ReqwestError,
};
use actix_web::{error, web, Error as ActixError, HttpResponse};
use futures::Future;
use graphql_client::GraphQLQuery;
use merit::IconBuilder;
use reqwest::r#async as req;
use serde;
use serde_derive::Deserialize;
use std::{env, str};

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "src/resx/github_schema.graphql",
  query_path = "src/resx/github_query.graphql",
  response_derives = "Debug"
)]
struct GithubLicense;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "src/resx/github_schema.graphql",
  query_path = "src/resx/github_query.graphql",
  response_derives = "Debug"
)]
struct GithubStarCount;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "src/resx/github_schema.graphql",
  query_path = "src/resx/github_query.graphql",
  response_derives = "Debug"
)]
struct GithubWatchCount;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "src/resx/github_schema.graphql",
  query_path = "src/resx/github_query.graphql",
  response_derives = "Debug"
)]
struct GithubForkCount;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "src/resx/github_schema.graphql",
  query_path = "src/resx/github_query.graphql",
  response_derives = "Debug"
)]
struct GithubTag;

#[derive(Deserialize, Debug, Clone)]
struct GithubParams {
  owner: String,
  name: String,
  tag_name: Option<String>,
}

static GITHUB_GQL_ENDPOINT: &'static str = "https://api.github.com/graphql";

fn make_req<T, Q>(client: &req::Client, query: &Q) -> impl Future<Item = T, Error = ActixError>
where
  for<'de> T: serde::Deserialize<'de>,
  Q: serde::Serialize,
{
  client
    .post(GITHUB_GQL_ENDPOINT)
    .bearer_auth(env::var("GH_ACCESS_TOKEN").unwrap())
    .json(&query)
    .send()
    .and_then(|mut resp: req::Response| resp.json::<T>())
    .map_err(ReqwestError::from)
    .map_err(ActixError::from)
}

fn github_lic_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<GithubParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let q = GithubLicense::build_query(github_license::Variables {
    owner: String::from(&params.owner),
    name: String::from(&params.name),
  });
  make_req(&client, &q)
    .and_then(|rd: github_license::ResponseData| {
      let lic_str = rd
        .repository
        .map(|lic| lic.license_info.map(|spdx| spdx.spdx_id))
        .flatten()
        .flatten()
        .unwrap_or("no license".to_string());
      Ok(lic_str)
    })
    .and_then(move |lic_str| {
      let mut badge = create_badge("license", &lic_str, None, &query);
      let icon = IconBuilder::new("github").build();
      badge.icon(icon);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn github_stars_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<GithubParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let q = GithubStarCount::build_query(github_star_count::Variables {
    owner: String::from(&params.owner),
    name: String::from(&params.name),
  });
  make_req(&client, &q)
    .and_then(|rd: github_star_count::ResponseData| {
      let star_count = rd
        .repository
        .map(|repo| repo.stargazers.total_count)
        .unwrap_or(0);
      Ok(star_count)
    })
    .and_then(move |star_count| {
      let count = star_count.to_string();
      let mut badge = create_badge("stars", &count, None, &query);
      let icon = IconBuilder::new("github").build();
      badge.icon(icon);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn github_watch_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<GithubParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let q = GithubWatchCount::build_query(github_watch_count::Variables {
    owner: String::from(&params.owner),
    name: String::from(&params.name),
  });
  make_req(&client, &q)
    .and_then(|rd: github_watch_count::ResponseData| {
      let star_count = rd
        .repository
        .map(|repo| repo.watchers.total_count)
        .unwrap_or(0);
      Ok(star_count)
    })
    .and_then(move |star_count| {
      let count = star_count.to_string();
      let mut badge = create_badge("watchers", &count, None, &query);
      let icon = IconBuilder::new("github").build();
      badge.icon(icon);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn github_fork_handler(
  client: web::Data<req::Client>,
  (params, query): (web::Path<GithubParams>, web::Query<QueryInfo>),
) -> impl Future<Item = HttpResponse, Error = ActixError> {
  let q = GithubForkCount::build_query(github_fork_count::Variables {
    owner: String::from(&params.owner),
    name: String::from(&params.name),
  });
  make_req(&client, &q)
    .and_then(|rd: github_fork_count::ResponseData| {
      let star_count = rd.repository.map(|repo| repo.fork_count).unwrap_or(0);
      Ok(star_count)
    })
    .and_then(move |star_count| {
      let count = star_count.to_string();
      let mut badge = create_badge("forks", &count, None, &query);
      let icon = IconBuilder::new("github").build();
      badge.icon(icon);
      let svg = badge.to_string();
      Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
    })
}

fn not_impl() -> impl Future<Item = (), Error = ActixError> {
  futures::done(Err(error::ErrorNotImplemented("No ready yet")))
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.data(req::Client::new()).service(
    web::scope("/github/{owner}/{name}")
      .route("/lic", web::get().to_async(github_lic_handler))
      .route("/stars", web::get().to_async(github_stars_handler))
      .route("/watches", web::get().to_async(github_watch_handler))
      .route("/forks", web::get().to_async(github_fork_handler))
      .route("/release/{tag_name}", web::get().to_async(not_impl)),
  );
}
