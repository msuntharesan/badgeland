#![feature(rustc_private)]
#![feature(option_flattening)]

#[macro_use]
extern crate actix_web;

mod badge_routes;
mod services;
mod utils;

use actix_files as fs;
use actix_web::{middleware, App, HttpServer, Result};
use dotenv::dotenv;
use env_logger::Env;
use listenfd::ListenFd;
use std::io;

#[get("/")]
pub fn index() -> Result<fs::NamedFile> {
  Ok(fs::NamedFile::open("static/index.html")?)
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
      .service(index)
      .configure(badge_routes::config)
      .configure(services::npm::config)
      .service(fs::Files::new("/static", "static/"))
  });

  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l).unwrap()
  } else {
    let addr = "127.0.0.1:3000";
    println!("Listening on {}", addr);
    server.bind(addr).unwrap()
  };
  server.start();
  sys.run()
}
