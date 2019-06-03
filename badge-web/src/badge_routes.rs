use super::services::utils::badge_query::{BadgeSize, QueryInfo};
use actix_web::{web, FromRequest, HttpRequest, HttpResponse};
use badger::{Badge, Size, Styles};
use serde_derive::Deserialize;
use std::str;

#[derive(Deserialize)]
struct BadgeInfo {
  text: Option<String>,
  subject: String,
  color: Option<String>,
}

fn badge_handler(req: HttpRequest) -> HttpResponse {
  let params = web::Path::<BadgeInfo>::extract(&req).unwrap();
  let query = web::Query::<QueryInfo>::extract(&req).unwrap();

  let mut req_badge = Badge::new(&params.subject);
  if let Some(text) = &params.text {
    req_badge.text(text);
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
  match (&query.icon, &query.icon_color) {
    (Some(i), Some(c)) => {
      req_badge.icon(i, Some(c));
    }
    (Some(i), None) => {
      req_badge.icon(i, None);
    }
    (_, _) => (),
  }

  if let Some(bs) = &query.size {
    req_badge.size(match bs {
      BadgeSize::Large => Size::Large,
      BadgeSize::Medium => Size::Medium,
      BadgeSize::Small => Size::Small,
    });
  }
  HttpResponse::Ok()
    .content_type("image/svg+xml")
    .body(req_badge.to_string())
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/badge/{subject}", web::get().to(badge_handler))
    .route("/badge/{subject}/{text}", web::get().to(badge_handler))
    .route("/badge/{subject}/{text}/{color}", web::get().to(badge_handler));
}
