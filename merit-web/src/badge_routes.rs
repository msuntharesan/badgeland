use super::utils::error::BadgeError;
use super::utils::{BadgeOptions, QueryInfo};
use actix_web::{
  http::{StatusCode, Uri},
  web, HttpRequest, HttpResponse,
};
use awc::Client;
use merit::{Badge, BadgeData, Icon, Size, Style};
use serde::Deserialize;
use std::convert::TryFrom;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::resource("/url").route(web::get().to(url_badge_handler)))
    .service(
      web::scope("/b")
        .route("/{text}", web::get().to(badge_handler))
        .route("/{subject}/{text}", web::get().to(badge_handler)),
    );
}

async fn url_badge_handler(req: HttpRequest, query: web::Query<QueryInfo>) -> Result<HttpResponse, BadgeError> {
  let query: QueryInfo = query.into_inner();
  let url = query.source;

  let url = url
    .ok_or("source query param missing".to_string())
    .and_then(|u| u.parse::<Uri>().map_err(|e| e.to_string()))
    .map_err(|e| BadgeError::Http {
      status: StatusCode::BAD_REQUEST,
      description: e,
      url: Some(req.uri().to_string()),
    })?;

  let client = Client::default();
  let mut resp = client
    .get(url)
    .header("accept", "application/json")
    .send()
    .await
    .map_err(BadgeError::from)?;

  dbg!(&resp);
  let data: BadgeOptions = resp.json().await?;

  let mut badge = Badge::new();
  badge.subject(&data.subject);

  match (data.color, query.color) {
    (_, Some(c)) | (Some(c), _) => {
      badge.color(c);
    }
    _ => {}
  }

  let icon = match (&data.icon, &query.icon) {
    (_, Some(i)) | (Some(i), _) => Icon::try_from(i.as_str()).ok(),
    _ => None,
  };

  if let Some(i) = icon {
    badge.icon(i);
  }

  let size = match (data.size, query.size) {
    (_, Some(s)) | (Some(s), _) => s,
    _ => Size::Medium,
  };
  badge.size(size);

  let style = match (data.style, query.style) {
    (_, Some(s)) | (Some(s), _) => s,
    _ => Style::Classic,
  };
  badge.style(style);

  let badge_svg = match (data.data, &data.text) {
    (Some(d), _) => badge.data(&d.0).to_string(),
    (_, Some(t)) => badge.text(t).to_string(),
    _ => badge.to_string(),
  };

  Ok(HttpResponse::Ok().content_type("image/svg+xml").body(badge_svg))
}

#[derive(Deserialize)]
struct BadgeInfo {
  text: String,
  subject: Option<String>,
}

fn badge_handler((params, query): (web::Path<BadgeInfo>, web::Query<QueryInfo>)) -> HttpResponse {
  let query = query.into_inner();
  let mut req_badge = Badge::new();
  if let Some(c) = &params.subject {
    req_badge.subject(c);
  }

  if let Some(c) = query.color {
    req_badge.color(c);
  }

  if let Some(s) = query.style {
    req_badge.style(s);
  }

  if let Some(i) = &query.icon {
    let icon = Icon::try_from(i.as_str());
    if let Ok(i) = icon {
      req_badge.icon(i);
    }
    if let Some(ic) = query.icon_color {
      req_badge.icon_color(ic);
    }
  }

  if let Some(bs) = query.size {
    req_badge.size(bs);
  }

  let badge_svg = match params.text.parse::<BadgeData>() {
    Ok(data) if data.0.len() > 1 => req_badge.data(&data.0).to_string(),
    _ => req_badge.text(&params.text).to_string(),
  };

  HttpResponse::Ok().content_type("image/svg+xml").body(badge_svg)
}