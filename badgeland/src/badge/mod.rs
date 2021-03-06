mod content;

use super::{icons::Icon, Color, DEFAULT_BLACK, DEFAULT_BLUE, DEFAULT_GRAY, DEFAULT_GRAY_DARK, DEFAULT_WHITE};
use content::{BadgeContentSize, ContentSize, Path, TextWidth};
use core::f32;
use maud::{html, PreEscaped};
use std::{fmt, str::FromStr};

#[cfg(feature = "serde_de")]
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub enum Style {
    Classic,
    Flat,
}

const CLASSIC_STYLE_GRADIENT: PreEscaped<&'static str> = PreEscaped(
    r##"<linearGradient id="a" x2="0" y2="75%"><stop offset="0" stop-color="#eee" stop-opacity="0.1"></stop><stop offset="1" stop-opacity="0.3"></stop></linearGradient>"##,
);

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

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde_de", derive(Serialize))]
pub enum Size {
    Large,
    Medium,
    Small,
}

#[cfg(feature = "serde_de")]
impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Size::from_str(s.as_str()).map_err(de::Error::custom)
    }
}

impl FromStr for Size {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "large" | "l" => Ok(Size::Large),
            "medium" | "m" => Ok(Size::Medium),
            "small" | "s" => Ok(Size::Small),
            _ => Err(format!("'{}' is not a valid value for Size", s)),
        }
    }
}

#[derive(Debug)]
pub struct BadgeTypeInit;
#[derive(Debug)]
pub struct BadgeTypeData<'a>(&'a [f32]);
#[derive(Debug)]
pub struct BadgeTypeText<'a>(&'a str);

pub trait BadgeType<'a> {
    fn content(&self) -> BadgeContentTypes;
}

#[derive(Debug)]
pub enum BadgeContentTypes<'a> {
    None,
    Text(&'a str),
    Data(&'a [f32]),
}

impl BadgeContentTypes<'_> {
    #[inline]
    fn is_some(&self) -> bool {
        !matches!(self, BadgeContentTypes::None)
    }
}

impl BadgeType<'_> for BadgeTypeInit {
    #[inline]
    fn content(&self) -> BadgeContentTypes {
        BadgeContentTypes::None
    }
}

impl<'a> BadgeType<'a> for BadgeTypeData<'a> {
    #[inline]
    fn content(&self) -> BadgeContentTypes {
        BadgeContentTypes::Data(self.0)
    }
}

impl<'a> BadgeType<'a> for BadgeTypeText<'a> {
    #[inline]
    fn content(&self) -> BadgeContentTypes {
        BadgeContentTypes::Text(self.0)
    }
}

#[derive(Debug)]
pub struct Badge<'a, S: BadgeType<'a>> {
    subject: Option<&'a str>,
    color: Color,
    style: Style,
    icon: Option<Icon<'a>>,
    icon_color: Color,
    size: Size,
    content: S,
}

impl<'a> Badge<'a, BadgeTypeInit> {
    pub fn new() -> Self {
        Badge {
            subject: None,
            color: DEFAULT_BLUE.parse().unwrap(),
            style: Style::Classic,
            icon: None,
            icon_color: DEFAULT_WHITE.parse().unwrap(),
            size: Size::Small,
            content: BadgeTypeInit,
        }
    }

    pub fn subject(&mut self, subject: &'a str) -> &mut Self {
        self.subject = Some(subject);
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn icon(&mut self, icon: Icon<'a>) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = size;
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn icon_color(&mut self, c: Color) -> &mut Self {
        if let Some(_) = &self.icon {
            self.icon_color = c;
        }
        self
    }

    pub fn text(self, text: &'a str) -> Badge<'a, BadgeTypeText<'a>> {
        Badge {
            subject: self.subject,
            color: self.color,
            style: self.style,
            icon: self.icon,
            icon_color: self.icon_color,
            size: self.size,
            content: BadgeTypeText(text),
        }
    }

    pub fn data(self, data: &'a [f32]) -> Badge<'a, BadgeTypeData<'a>> {
        Badge {
            subject: self.subject,
            color: self.color,
            style: self.style,
            icon: self.icon,
            icon_color: self.icon_color,
            size: self.size,
            content: BadgeTypeData(data),
        }
    }
}

impl<'a, T: BadgeType<'a>> Badge<'a, T> {
    #[inline]
    fn get_height(&self) -> usize {
        match self.size {
            Size::Small => 20,
            Size::Medium => 30,
            Size::Large => 40,
        }
    }

    #[inline]
    fn get_icon_size(&self) -> (usize, usize) {
        match (&self.icon, self.size) {
            (Some(_), Size::Large) => (30, 10),
            (Some(_), Size::Medium) => (20, 8),
            (Some(_), Size::Small) => (15, 5),
            _ => (0, 0),
        }
    }
}

const SVG_FONT_MULTIPLIER: f32 = 0.65;

impl<'a, T: BadgeType<'a>> fmt::Display for Badge<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self.get_height();

        let font_size = height as f32 * SVG_FONT_MULTIPLIER;

        let padding = height / 2;

        let (icon_width, x_offset) = self.get_icon_size();

        let subject_size: ContentSize = match self.subject {
            Some(s) => s.content_size(height, s.get_text_width(font_size), padding, x_offset + icon_width),
            None if self.icon.is_some() => ContentSize {
                rw: icon_width + x_offset * 2,
                x: x_offset,
                y: height,
            },
            _ => ContentSize::default(),
        };

        let content = self.content.content();

        let content_size = match content {
            BadgeContentTypes::Data(d) => d.content_size(height, height * 5, padding, 0),
            BadgeContentTypes::Text(c) => c.content_size(height, c.get_text_width(font_size), padding, 0),
            _ => ContentSize::default(),
        };

        let path_str = if let BadgeContentTypes::Data(d) = content {
            Path::new(d, height, height * 5)
                .into_iter()
                .enumerate()
                .map(|(i, (x, y))| match i {
                    0 => format!("M0 {y}L{x} {y}", x = x, y = y),
                    _ => format!("L{x} {y}", x = x, y = y),
                })
                .collect::<String>()
        } else {
            String::new()
        };

        let width = subject_size.rw + content_size.rw;

        let rx = match self.size {
            Size::Medium => 6,
            Size::Large => 9,
            _ => 3,
        };

        let markup = html! {
          svg
            xmlns:xlink="http://www.w3.org/1999/xlink"
            xmlns="http://www.w3.org/2000/svg"
            viewBox={(format!("0 0 {} {}", width, height))}
            height=(height)
            width=(width) {
              defs {
                @if let Some(icon) = &self.icon { (PreEscaped(icon.symbol())) }
                @if matches!(self.style, Style::Classic) {
                  (CLASSIC_STYLE_GRADIENT)
                }
                mask id="bg-mask" {
                  rect fill=(DEFAULT_WHITE) height=(height) rx=(rx) width=(width) {}
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
              g#bg mask=@if self.style == Style::Classic { "url(#bg-mask)" } {
                rect fill=@if self.style == Style::Flat { (DEFAULT_GRAY) } @else { "url(#a)" } height=(height) width=(width) {}
                @if self.subject.is_some() || self.icon.is_some() {
                  rect#subject
                    fill=@if content.is_some() { (DEFAULT_GRAY_DARK) } @else { (self.color.0) }
                    height=(height)
                    width=(subject_size.rw)
                    {}
                }
                rect#content
                  fill=@match &content{
                    BadgeContentTypes::Data(_) => { (DEFAULT_GRAY) }
                    _ => (self.color.0)
                  }
                  height=(height)
                  width=(content_size.rw)
                  x=(subject_size.rw)
                  {}
              }
              g#text
                fill=(DEFAULT_WHITE)
                font-family="Verdana,sans-serif"
                font-size=(font_size)
                transform="translate(0, 0)" {
                  @if let Some(icon) = &self.icon {
                    use
                      filter="url(#shadow)"
                      xlink:href={"#" (icon.name())}
                      x=(x_offset)
                      y=(((height - icon_width) / 2))
                      width=(icon_width)
                      height=(icon_width)
                      fill=(self.icon_color.0)
                      {}
                  }
                  @if let Some(s) = self.subject {
                    text
                    dominant-baseline="middle"
                    text-anchor="middle"
                    x=(subject_size.x)
                      y=(subject_size.y)
                      filter="url(#shadow)"
                      { (s) }
                  }
                  @match content {
                    BadgeContentTypes::Data(_) => {
                      path
                        fill="none"
                        transform=(format!("translate({}, {})", subject_size.rw, 0))
                        stroke=(self.color.0)
                        stroke-width="1px"
                        d=(&path_str)
                        {}
                      path
                        fill=(self.color.0)
                        fill-opacity="0.2"
                        transform=(format!("translate({}, {})", subject_size.rw, 0))
                        stroke="none"
                        stroke-width="0px"
                        d=(format!("{}V{}H0Z", &path_str, height))
                        {}
                    }
                    BadgeContentTypes::Text(c) => {
                      text
                        x=((subject_size.rw + content_size.x))
                        y=(content_size.y)
                        text-anchor="middle"
                        dominant-baseline="middle"
                        filter="url(#shadow)"
                        { (c) }
                    }
                    _ => {}
                  }
              }
          }
        };

        write!(f, "{}", markup.into_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{Badge, Color, Size, Style, DEFAULT_BLUE};
    use scraper::{Html, Selector};

    use crate::Icon;
    use std::convert::TryFrom;

    #[test]
    fn default_badge_has_classic_style() {
        let mut badge = Badge::new();
        &badge.subject("just text");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        assert_eq!(badge.style, Style::Classic, "style not Classic");
        let lg_selector = Selector::parse("linearGradient").unwrap();
        assert!(doc.select(&lg_selector).next().is_some());
    }
    #[test]
    fn default_badge_has_20px_height() {
        let mut badge = Badge::new();
        &badge.subject("just text");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        let selector = Selector::parse("svg").unwrap();
        let svg = doc.select(&selector).next().unwrap();
        assert_eq!(svg.value().attr("height"), Some("20"));
    }
    #[test]
    fn default_badge_only_has_subject() {
        let mut badge = Badge::new();
        &badge.subject("just subject");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        let text_sel = Selector::parse("g#text > text").unwrap();
        let text_els = doc.select(&text_sel);
        assert_eq!(text_els.count(), 1);
        let text = doc.select(&text_sel).next().unwrap();
        assert_eq!(text.text().collect::<String>(), String::from("just subject"));
    }
    #[test]
    fn default_badge_has_333_as_background_color() {
        let mut badge = Badge::new();
        &badge.subject("just text");
        badge.color(DEFAULT_BLUE.parse::<Color>().unwrap());
        let def_color: Color = DEFAULT_BLUE.parse().unwrap();
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        let rect_sel = Selector::parse("g#bg > rect#subject").unwrap();
        let rect = doc.select(&rect_sel).next().unwrap();
        assert_eq!(rect.value().attr("fill").unwrap(), &def_color.0);
    }

    #[test]
    fn badge_with_text() {
        let mut badge = Badge::new();
        badge.subject("with subject");
        let doc = Html::parse_fragment(&badge.text("badge text").to_string());
        let subject_sel = Selector::parse("g#text > text:last-child").unwrap();
        let subject = doc.select(&subject_sel).next().unwrap();
        assert_eq!(subject.text().collect::<String>(), String::from("badge text"));
    }

    #[test]
    #[cfg(feature = "static_icons")]
    fn badge_with_icon() {
        let icon = Icon::try_from("git").unwrap();
        let mut badge = Badge::new();
        &badge.subject("with icon").icon(icon);

        let icon = &badge.icon;
        assert!(icon.is_some());

        let doc = Html::parse_fragment(&badge.to_string());
        let icon_sel = Selector::parse("symbol").unwrap();
        let icon_symbol = doc.select(&icon_sel).next().unwrap();
        assert_eq!(icon_symbol.value().attr("id"), Some("git"));
    }

    #[test]
    #[cfg(feature = "static_icons")]
    fn badge_with_icon_only() {
        let icon = Icon::try_from("git").unwrap();
        let mut badge = Badge::new();
        &badge.icon(icon);

        let icon = &badge.icon;
        assert!(icon.is_some());

        let doc = Html::parse_fragment(&badge.to_string());
        let icon_sel = Selector::parse("symbol").unwrap();
        let icon_symbol = doc.select(&icon_sel).next().unwrap();
        assert_eq!(icon_symbol.value().attr("id"), Some("git"));
    }

    #[test]
    fn badge_has_medium_icon() {
        let mut badge = Badge::new();
        &badge.subject("with icon").size(Size::Medium);
        let doc = Html::parse_fragment(&badge.to_string());
        let svg_sel = Selector::parse("svg").unwrap();
        let svg = doc.select(&svg_sel).next().unwrap();
        assert_eq!(svg.value().attr("height"), Some("30"));
    }
    #[test]
    fn badge_has_large_icon() {
        let mut badge = Badge::new();
        &badge.subject("with icon").size(Size::Large);
        let doc = Html::parse_fragment(&badge.to_string());
        let svg_sel = Selector::parse("svg").unwrap();
        let svg = doc.select(&svg_sel).next().unwrap();
        assert_eq!(svg.value().attr("height"), Some("40"));
    }

    #[test]
    fn badge_with_data() {
        let mut badge = Badge::new();
        badge.subject("Some data");
        let badge = badge.data(&[1., 2., 3., 4., 5.]);

        let doc = Html::parse_fragment(&badge.to_string());
        println!("{:?}", &badge.to_string());
        let line_sel = Selector::parse("path").unwrap();
        let svg = doc.select(&line_sel).next().unwrap();
        assert!(svg.value().attr("d").is_some());
    }
}
