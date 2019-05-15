use actix_web::{web, FromRequest, HttpRequest, HttpResponse};
use badge_maker::{Badge, Size, Styles};
use serde_derive::Deserialize;
use std::str;

#[derive(Deserialize)]
struct BadgeInfo {
  text: String,
  subject: Option<String>,
  color: Option<String>,
  size: Option<BadgeSize>,
}

#[derive(Debug, Deserialize)]
enum BadgeSize {
  #[serde(alias = "large")]
  Large,
  #[serde(alias = "medium")]
  Medium,
  #[serde(alias = "small")]
  Small,
}

#[derive(Deserialize, Debug)]
struct QueryInfo {
  icon: Option<String>,
  color: Option<String>,
  style: Option<String>,
}

fn badge_handler(req: HttpRequest) -> HttpResponse {
  let params = web::Path::<BadgeInfo>::extract(&req).unwrap();
  let query = web::Query::<QueryInfo>::extract(&req).unwrap();

  let mut req_badge = Badge::new(&params.text);
  if let Some(subject) = &params.subject {
    req_badge.subject(&subject);
  }
  if let Some(c) = &params.color {
    req_badge.color(c);
  }

  match &query.style {
    Some(x) if x == "flat" => {
      req_badge.style(Styles::Flat);
    }
    _ => {}
  }
  match (&query.icon, &query.color) {
    (Some(i), Some(c)) => {
      req_badge.icon(i, Some(c));
    }
    (Some(i), None) => {
      req_badge.icon(i, None);
    }
    (_, _) => (),
  }

  if let Some(bs) = &params.size {
    req_badge.size(match bs {
      BadgeSize::Large => Size::Large,
      BadgeSize::Medium => Size::Medium,
      BadgeSize::Small => Size::Small,
    });
  }

  HttpResponse::Ok()
    .content_type("image/svg+xml")
    .body(req_badge.to_svg())
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/badge")
      .service(web::resource("/{text}").route(web::get().to(badge_handler)))
      .service(web::resource("/{subject}/{text}").route(web::get().to(badge_handler)))
      .service(web::resource("/{subject}/{text}/{color}").route(web::get().to(badge_handler)))
      .service(
        web::resource("/{subject}/{text}/{color}/{size}").route(web::get().to(badge_handler)),
      ),
  );
}
