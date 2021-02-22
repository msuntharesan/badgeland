// #![feature(rustc_private)]

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
use badgeland::Badge;
use std::{env, io};

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::build(StatusCode::TEMPORARY_REDIRECT)
    .header(header::LOCATION, "https://github.com/msuntharesan/badgeland")
    .finish()
}

async fn default_404() -> impl Responder {
  let mut badge = Badge::new();
  badge.subject("Error").color("red".parse().unwrap());

  HttpResponse::NotFound()
    .content_type("image/svg+xml")
    .body(badge.text("404").to_string())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  let env = Env::new().filter("RUST_LOG");
  env_logger::init_from_env(env);

  let mut server = HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::new("%a %r %s %Dms %b %{Referer}i %{User-Agent}i"))
      .wrap(middleware::NormalizePath::default())
      .default_service(web::route().to(default_404))
      .service(index)
      .configure(badge_routes::config)
      .service(
        web::scope("/")
          .wrap(middleware::DefaultHeaders::new().header("Cache-Control", format!("public, max-age={}", 60 * 24 * 100)))
          .service(Files::new("/", format!("{}/static/", env!("CARGO_MANIFEST_DIR"))).prefer_utf8(true)),
      )
  });

  server = if let Some(l) = ListenFd::from_env().take_tcp_listener(0)? {
    server.listen(l)?
  } else {
    let port = env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{}", port);
    server.bind(addr)?
  };
  server.run().await
}
