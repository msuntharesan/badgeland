use super::utils::error::BadgeError;
use super::utils::{QueryInfo, BadgeOptions};
use actix_web::{
  http::{StatusCode, Uri},
  web, HttpRequest, HttpResponse,
};
use awc::Client;
use merit::{Badge, BadgeData, Icon, Size, Styles};
use serde::Deserialize;
use url::form_urlencoded;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/badge")
      .route("/url/{url:.*}", web::get().to(url_badge_handler))
      .route("/{subject}", web::get().to(badge_handler))
      .route("/{subject}/{text}", web::get().to(badge_handler)),
  );
}

async fn url_badge_handler(
  req: HttpRequest,
  (params, query): (web::Path<String>, web::Query<QueryInfo>),
) -> Result<HttpResponse, BadgeError> {
  let parsed = form_urlencoded::parse(&params.as_bytes())
    .into_owned()
    .collect::<Vec<_>>();

  let (url, _) = parsed.first().ok_or(BadgeError::default())?;

  let url = url.parse::<Uri>().map_err(|e| BadgeError::Http {
    status: StatusCode::BAD_REQUEST,
    description: e.to_string(),
    url: Some(req.uri().to_string()),
  })?;

  let client = Client::default();
  let mut b = client
    .get(url)
    .header("accept", "application/json")
    .send()
    .await
    .map_err(BadgeError::from)?;
  println!("{:?}", b.status());
  let data: BadgeOptions = b.json().await?;

  let mut badge = Badge::new(&data.subject);

  match (&data.color, &query.color) {
    (None, Some(c)) => {
      badge.color(c);
    }
    (Some(c), None) => {
      badge.color(c);
    }
    _ => {}
  }

  let mut icon = match (&data.icon, &query.icon) {
    (_, Some(i)) => Some(Icon::new(i)),
    (Some(i), _) => Some(Icon::new(i)),
    _ => None,
  };

  if let Some(i) = &mut icon {
    match (&data.icon_color, &query.icon_color) {
      (None, Some(c)) => {
        i.color(c);
      }
      (Some(c), None) => {
        i.color(c);
      }
      _ => {}
    }
    if let Some(i) = i.build() {
      badge.icon(i);
    }
  }

  let size = match (data.size, query.size) {
    (_, Some(s)) => s,
    (Some(s), _) => s,
    _ => Size::Medium,
  };
  badge.size(size);

  let style = match (data.style, query.style) {
    (_, Some(s)) => s,
    (Some(s), _) => s,
    _ => Styles::Classic,
  };
  badge.style(style);

  let badge_svg = match (data.data, &data.text) {
    (Some(d), _) => badge.data(d.0).to_string(),
    (_, Some(t)) => badge.text(t).to_string(),
    _ => badge.to_string(),
  };

  Ok(HttpResponse::Ok().content_type("image/svg+xml").body(badge_svg))
}

#[derive(Deserialize)]
struct BadgeInfo {
  text: Option<String>,
  subject: String,
}

fn badge_handler((params, query): (web::Path<BadgeInfo>, web::Query<QueryInfo>)) -> HttpResponse {
  let mut req_badge = Badge::new(&params.subject);
  if let Some(c) = &query.color {
    req_badge.color(c);
  }

  if let Some(s) = &query.style {
    req_badge.style(*s);
  }

  if let Some(i) = &query.icon {
    let mut icon = Icon::new(i);
    if let Some(ic) = &query.icon_color {
      icon.color(ic);
    }
    if let Some(i) = icon.build() {
      req_badge.icon(i);
    }
  }

  if let Some(bs) = &query.size {
    req_badge.size(*bs);
  }

  let badge_svg = if let Some(text) = &params.text {
    match text.parse::<BadgeData>() {
      Ok(data) if data.0.len() > 1 => req_badge.data(data.0).to_string(),
      _ => req_badge.text(text).to_string(),
    }
  } else {
    req_badge.to_string()
  };
  HttpResponse::Ok().content_type("image/svg+xml").body(badge_svg)
}
