#![feature(rustc_private)]
#[macro_use]
extern crate actix_web;
extern crate serde_derive;

mod badge_routes;

use actix_files as fs;
use actix_web::{middleware, App, HttpServer, Result};
use listenfd::ListenFd;
use std::io;

#[get("/")]
pub fn index() -> Result<fs::NamedFile> {
  Ok(fs::NamedFile::open("static/index.html")?)
}

fn main() -> io::Result<()> {
  let mut listenfd = ListenFd::from_env();

  let sys = actix_rt::System::new("badge");

  let mut server = HttpServer::new(move || {
    App::new()
      .wrap(middleware::NormalizePath)
      .service(index)
      .configure(badge_routes::config)
      .service(fs::Files::new("/static", "static/"))
  });

  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l).unwrap()
  } else {
    server.bind("127.0.0.1:3000").unwrap()
  };

  server.start();
  sys.run()
}