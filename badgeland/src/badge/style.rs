use super::{content::ContentSize, BadgeContentType};
use super::{DEFAULT_BLACK, DEFAULT_GRAY, DEFAULT_GRAY_DARK, DEFAULT_WHITE};
use crate::{Color, Icon};
use maud::PreEscaped;
use maud::{html, Markup};
use std::str::FromStr;

#[cfg(feature = "serde_de")]
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub enum Style {
    Classic,
    Flat,
}

impl Default for Style {
    fn default() -> Self {
        Style::Classic
    }
}

#[cfg(feature = "serde_de")]
impl<'de> Deserialize<'de> for Style {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Style::from_str(&s).map_err(|e| de::Error::custom(e))
    }
}

impl FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "classic" | "c" => Ok(Style::Classic),
            "flat" | "f" => Ok(Style::Flat),
            _ => Err(format!("'{}' is not a valid value for Styles", s)),
        }
    }
}

fn content_template<'a>(
    height: usize,
    font_size: f32,
    x_offset: usize,

    icon: Option<(&'a Icon<'a>, &'a Color)>,

    icon_width: usize,

    color: &'a Color,

    content: BadgeContentType<'a>,
    content_size: ContentSize,

    subject: Option<&'a str>,
    subject_size: ContentSize,
) -> Markup {
    html!(
        g#text
            fill=(DEFAULT_WHITE)
            font-family="Verdana,sans-serif"
            font-size=(font_size)
            transform="translate(0, 0)" {
                @if let Some((icon, icon_color)) = &icon {
                use
                    filter="url(#shadow)"
                    xlink:href={"#" (icon.name())}
                    x=(x_offset)
                    y=(((height - icon_width) / 2))
                    width=(icon_width)
                    height=(icon_width)
                    fill=(icon_color) {}
                }
                @if let Some(s) = subject {
                text
                    dominant-baseline="middle"
                    text-anchor="middle"
                    x=(subject_size.x)
                    y=(subject_size.y)
                    filter="url(#shadow)"
                    { (s) }
                }
                @if let BadgeContentType::Text(c) = content {
                    text
                        x=((subject_size.rw + content_size.x))
                        y=(content_size.y)
                        text-anchor="middle"
                        dominant-baseline="middle"
                        filter="url(#shadow)"
                        { (c) }
                    }
                @if let Some(path_str) = content.path_str(height, height * 5) {
                    path
                        fill="none"
                        transform=(format!("translate({}, {})", subject_size.rw, 0))
                        stroke=(color)
                        stroke-width="1px"
                        d=(&path_str) {}
                    path
                        fill=(color)
                        fill-opacity="0.2"
                        transform=(format!("translate({}, {})", subject_size.rw, 0))
                        stroke="none"
                        stroke-width="0px"
                        d=(format!("{}V{}H0Z", &path_str, height)) {}
                }
        }
    )
}

pub(super) fn classic_template<'a>(
    width: usize,
    height: usize,
    font_size: f32,
    x_offset: usize,
    rx: usize,

    icon: Option<(&'a Icon<'a>, &'a Color)>,

    icon_width: usize,

    color: &'a Color,

    content: BadgeContentType<'a>,
    content_size: ContentSize,

    subject: Option<&'a str>,
    subject_size: ContentSize,
) -> Markup {
    html!(
        svg
            xmlns:xlink="http://www.w3.org/1999/xlink"
            xmlns="http://www.w3.org/2000/svg"
            viewBox={(format!("0 0 {} {}", width, height))}
            height=(height)
            width=(width) {
                defs {
                    @if let Some((icon, _)) = &icon { (PreEscaped(icon.symbol())) }
                    linearGradient id="a" x2="0" y2="75%"{
                        stop offset="0" stop-color="#eee" stop-opacity="0.1" {}
                        stop offset="1" stop-opacity="0.3" {}
                    }
                    mask id="bg-mask" {
                      rect fill=(DEFAULT_WHITE) height=(height) rx=(rx) width=(width){}
                    }
                    filter id="shadow" {
                      feDropShadow
                        dx="-0.8"
                        dy="-0.8"
                        stdDeviation="0"
                        flood-color=@if content.is_some() { (DEFAULT_BLACK) } @else { (DEFAULT_GRAY_DARK) }
                        flood-opacity="0.4" {}
                    }
                }
                g#bg mask="url(#bg-mask)" {
                    rect fill="url(#a)" height=(height) width=(width) {}
                    @if subject.is_some() || icon.is_some() {
                      rect#subject
                        fill=@if content.is_some() { (DEFAULT_GRAY_DARK) } @else { (color) }
                        height=(height)
                        width=(subject_size.rw) {}
                    }
                    rect#content
                        fill=@match &content{
                            BadgeContentType::Data(_) => { (DEFAULT_GRAY) }
                            _ => (color)
                        }
                        height=(height)
                        width=(content_size.rw)
                        x=(subject_size.rw) {}
                }
                (content_template(
                    height,
                    font_size,
                    x_offset,

                    icon,

                    icon_width,

                    color,

                    content,
                    content_size,

                    subject,
                    subject_size,
                ))
        }
    )
}

pub(super) fn flat_template<'a>(
    width: usize,
    height: usize,
    font_size: f32,
    x_offset: usize,

    icon: Option<(&'a Icon<'a>, &'a Color)>,

    icon_width: usize,

    color: &'a Color,

    content: BadgeContentType<'a>,
    content_size: ContentSize,

    subject: Option<&'a str>,
    subject_size: ContentSize,
) -> Markup {
    html!(
        svg
            xmlns:xlink="http://www.w3.org/1999/xlink"
            xmlns="http://www.w3.org/2000/svg"
            viewBox={(format!("0 0 {} {}", width, height))}
            height=(height)
            width=(width) {
                defs {
                    @if let Some((icon, _)) = &icon { (PreEscaped(icon.symbol())) }
                    filter id="shadow" {
                      feDropShadow
                        dx="-0.8"
                        dy="-0.8"
                        stdDeviation="0"
                        flood-color=@if content.is_some() { (DEFAULT_BLACK) } @else { (DEFAULT_GRAY_DARK) }
                        flood-opacity="0.4" {}
                    }
                }
                g#bg {
                    rect fill=(DEFAULT_GRAY) height=(height) width=(width) {}
                    @if subject.is_some() || icon.is_some() {
                      rect#subject
                        fill=@if content.is_some() { (DEFAULT_GRAY_DARK) } @else { (color) }
                        height=(height)
                        width=(subject_size.rw) {}
                    }
                    rect#content
                        fill=@match &content{
                            BadgeContentType::Data(_) => { (DEFAULT_GRAY) }
                            _ => (color)
                        }
                        height=(height)
                        width=(content_size.rw)
                        x=(subject_size.rw) {}
                }
                (content_template(
                    height,
                    font_size,
                    x_offset,

                    icon,

                    icon_width,

                    color,

                    content,
                    content_size,

                    subject,
                    subject_size,
                ))
        }
    )
}