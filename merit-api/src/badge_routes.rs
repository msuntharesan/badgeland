use super::utils::merit_query::{BadgeSize, QueryInfo};
use actix_web::{web, HttpResponse};
use merit::{Badge, BadgeData, Icon, Size, Styles};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct BadgeInfo {
  text: Option<String>,
  subject: String,
}

fn badge_handler((params, query): (web::Path<BadgeInfo>, web::Query<QueryInfo>)) -> HttpResponse {
  let mut req_badge = Badge::new(&params.subject);

  if let Some(text) = &params.text {
    if let Ok(data) = text.parse::<BadgeData>() {
      req_badge.data(data.0);
    } else {
      req_badge.text(text);
    }
  }
  if let Some(c) = &query.color {
    req_badge.color(c);
  }

  match &query.style {
    Some(x) if x == "flat" => {
      req_badge.style(Styles::Flat);
    }
    _ => {}
  }
  if let Some(i) = query.icon.as_ref() {
    let mut icon = Icon::new(i);
    if let Some(ic) = &query.icon_color {
      icon.set_color(ic);
    }
    req_badge.icon(icon.build());
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
    .route("/badge/{subject}/{text}", web::get().to(badge_handler));
}
