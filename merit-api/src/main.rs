#![feature(rustc_private)]

#[macro_use]
extern crate actix_web;

mod badge_routes;
mod services;
mod utils;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use env_logger::Env;
use listenfd::ListenFd;
use std::{env, io};
use utils::merit_query::*;

#[get("/")]
fn index() -> Result<fs::NamedFile> {
  Ok(fs::NamedFile::open("static/index.html")?)
}

fn default_404(query: web::Query<QueryInfo>) -> Result<HttpResponse> {
  let badge = create_badge("Error", "404", Some("grey"), &query);

  Ok(
    HttpResponse::NotFound()
      .content_type("image/svg+xml")
      .body(badge.to_string()),
  )
}

fn main() -> io::Result<()> {
  dotenv().ok();
  let env = Env::new().filter("LOG_LEVEL");
  env_logger::init_from_env(env);

  let mut listenfd = ListenFd::from_env();

  let sys = actix_rt::System::new("badge");

  let mut server = HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .wrap(middleware::NormalizePath)
      .wrap(
        middleware::DefaultHeaders::new()
          .header("Cache-Control", format!("public, max-age={}", 60 * 24)),
      )
      .default_service(web::route().to(default_404))
      .service(index)
      .configure(badge_routes::config)
      .configure(services::crates_io::config)
      .configure(services::github::config)
      .configure(services::npm::config)
      .service(fs::Files::new("/static", "static/"))
  });

  server = if let Some(l) = listenfd.take_tcp_listener(0)? {
    server.listen(l).unwrap()
  } else {
    let port = env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on {}", addr);
    server.bind(addr)?
  };
  server.start();
  sys.run()
}
