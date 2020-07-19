#![feature(rustc_private)]

#[macro_use]
extern crate actix_web;

mod badge_routes;
mod utils;

use actix_files::Files;
use actix_web::{
  http::{header, StatusCode},
  middleware, web, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use env_logger::Env;
use listenfd::ListenFd;
use merit::{Badge, DEFAULT_GRAY};
use std::{env, io};

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::build(StatusCode::TEMPORARY_REDIRECT)
    .header(header::LOCATION, "https://github.com/msuntharesan/merit")
    .finish()
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
  let icon: &'static [u8] = include_bytes!("../static/favicon.ico");
  HttpResponse::Ok().content_type("image/x-icon").body(icon)
}

async fn default_404() -> impl Responder {
  let mut badge = Badge::new("Error");
  badge.color(DEFAULT_GRAY.parse().unwrap());

  HttpResponse::NotFound()
    .content_type("image/svg+xml")
    .body(badge.text("404").to_string())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  let env = Env::new().filter("LOG_LEVEL");
  env_logger::init_from_env(env);

  let mut listenfd = ListenFd::from_env();

  let mut server = HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %D"))
      .wrap(middleware::NormalizePath)
      .wrap(middleware::DefaultHeaders::new().header("Cache-Control", format!("public, max-age={}", 60 * 24)))
      .default_service(web::route().to(default_404))
      .service(index)
      .service(favicon)
      .configure(badge_routes::config)
      .service(Files::new("/", "./merit-web/dist").index_file("index.html"))
  });

  server = if let Some(l) = listenfd.take_tcp_listener(0)? {
    server.listen(l).unwrap()
  } else {
    let port = env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on {}", addr);
    server.bind(addr)?
  };
  server.run().await
}
