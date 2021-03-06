use super::utils::{error::BadgeError, BadgeOptions, QueryInfo};
use actix_web::{http, middleware, web, HttpRequest, HttpResponse};
use awc::Client;
use badgeland::{Badge, BadgeData, Icon, Size, Style};
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    convert::TryFrom,
    hash::{Hash, Hasher},
    str::from_utf8,
    time::Duration,
};

const MAX_AGE: u16 = 60 * 24;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(url_badge_handler).service(
        web::scope("/b/")
            .wrap(middleware::DefaultHeaders::new().header("Cache-Control", format!("public, max-age={}", MAX_AGE)))
            .route("/{text}/", web::get().to(badge_handler))
            .route("/{subject}/{text}/", web::get().to(badge_handler)),
    );
}

#[get("/url/")]
async fn url_badge_handler(req: HttpRequest, query: web::Query<QueryInfo>) -> Result<HttpResponse, BadgeError> {
    let query: QueryInfo = query.into_inner();
    let url = query.source;

    let url = url
        .ok_or("source query param missing".to_string())
        .and_then(|u| u.parse::<http::Uri>().map_err(|e| e.to_string()))
        .map_err(|e| BadgeError::Http {
            status: http::StatusCode::BAD_REQUEST,
            description: e,
            url: Some(req.uri().to_string()),
        })?;

    let client = Client::builder().timeout(Duration::from_secs(10)).finish();
    let mut resp = client
        .get(url)
        .header("accept", "application/json")
        .send()
        .await
        .map_err(BadgeError::from)?;

    let cache_headers = resp
        .headers()
        .iter()
        .filter_map(|(h, v)| match *h {
            http::header::ETAG | http::header::CACHE_CONTROL => Some((h.to_owned(), v.to_owned())),
            _ => None,
        })
        .collect::<Vec<_>>();

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

    let mut resp = HttpResponse::Ok();

    if cache_headers.iter().any(|(h, _)| h != http::header::ETAG) {
        let mut hasher = DefaultHasher::new();
        badge_svg.hash(&mut hasher);
        let hash = hasher.finish();
        resp.header(http::header::ETAG, format!("u:{:x}", hash));
    }

    if cache_headers.iter().any(|(h, _)| h != http::header::CACHE_CONTROL) {
        resp.header(http::header::CACHE_CONTROL, format!("public, max-age={}", MAX_AGE));
    }

    resp.content_type("image/svg+xml");

    for (h, v) in cache_headers.iter() {
        resp.set_header(h, from_utf8(v.as_bytes()).expect("Failed to read value"));
    }

    Ok(resp.body(badge_svg))
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

    let mut hasher = DefaultHasher::new();
    badge_svg.hash(&mut hasher);
    let hash = hasher.finish();

    HttpResponse::Ok()
        .set_header(http::header::ETAG, format!("b:{:x}", hash))
        .content_type("image/svg+xml")
        .body(badge_svg)
}
