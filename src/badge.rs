use super::{get_color, icons::Icon};
use fmt::Display;
use maud::html;
use rusttype::{point, Font, Scale};
use std::{fmt, str::FromStr};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, PartialEq)]
pub enum Styles {
  Flat,
  Classic,
}

impl FromStr for Styles {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_ref() {
      "flat" | "f" => Ok(Styles::Flat),
      "classic" | "c" => Ok(Styles::Classic),
      _ => Err(format!("'{}' is not a valid value for Styles", s)),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Size {
  Large,
  Medium,
  Small,
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

fn get_font() -> Font<'static> {
  let font_data: &[u8] = include_bytes!("./resx/Verdana.ttf");
  Font::try_from_bytes(font_data).expect("Error constructing Font")
}

#[derive(PartialEq, Eq)]
pub enum BadgeTypeState {
  Init,
  Data,
  Text,
}

#[derive(Debug)]
pub enum BadgeType<'a, const S: BadgeTypeState> {
  Init,
  Data(Vec<i64>),
  Text(&'a str),
}

trait GetBadgeType {
  type BadgeContent;
  fn get(&self) -> Option<Self::BadgeContent>;
}

impl<'a> GetBadgeType for BadgeType<'a, { BadgeTypeState::Init }> {
  type BadgeContent = ();
  fn get(&self) -> Option<Self::BadgeContent> {
    None
  }
}

impl<'a> GetBadgeType for BadgeType<'a, { BadgeTypeState::Data }> {
  type BadgeContent = Vec<i64>;
  fn get(&self) -> Option<Self::BadgeContent> {
    match self {
      BadgeType::Data(d) => Some(d.to_owned()),
      _ => None,
    }
  }
}

impl<'a> GetBadgeType for BadgeType<'a, { BadgeTypeState::Text }> {
  type BadgeContent = &'a str;
  fn get(&self) -> Option<Self::BadgeContent> {
    match self {
      BadgeType::Text(t) => Some(*t),
      _ => None,
    }
  }
}
#[derive(Debug)]
pub struct Badge<'a, const S: BadgeTypeState> {
  pub subject: &'a str,
  pub color: String,
  pub style: Styles,
  pub icon: Option<Icon<'a>>,
  pub height: u32,
  pub content: BadgeType<'a, S>,
}

impl<'a> Badge<'a, { BadgeTypeState::Init }> {
  pub fn new(subject: &'a str) -> Self {
    Badge {
      subject,
      color: "#08C".into(),
      style: Styles::Classic,
      icon: None,
      height: 20,
      content: BadgeType::Init,
    }
  }

  pub fn color(&mut self, color: &'a str) -> &mut Self {
    if let Some(c) = get_color(color) {
      self.color = c;
    }
    self
  }

  pub fn icon(&mut self, icon: Icon<'a>) -> &mut Self {
    self.icon = Some(icon);
    self
  }
  pub fn size(&mut self, size: Size) -> &mut Self {
    let height: u32 = match size {
      Size::Small => 20,
      Size::Medium => 30,
      Size::Large => 40,
    };
    self.height = height;
    self
  }
  pub fn style(&mut self, style: Styles) -> &mut Self {
    self.style = style;
    self
  }
  pub fn text(self, text: &'a str) -> Badge<'a, { BadgeTypeState::Text }> {
    Badge {
      subject: self.subject,
      color: self.color,
      style: self.style,
      icon: self.icon,
      height: self.height,
      content: BadgeType::Text(text),
    }
  }

  pub fn data(self, data: Vec<i64>) -> Badge<'a, { BadgeTypeState::Data }> {
    Badge {
      subject: self.subject,
      color: self.color,
      style: self.style,
      icon: self.icon,
      height: self.height,
      content: BadgeType::Data(data),
    }
  }
}

#[derive(Debug, Default)]
struct BadgeContent {
  content: String,
  width: u32,
  height: u32,
}

trait Content {
  fn content(&self, height: u32) -> BadgeContent;
}

impl<'a> Content for Vec<i64> {
  fn content(&self, height: u32) -> BadgeContent {
    let width = if self.len() > 0 { height * 5 } else { 0 };
    let chart_height = height as f32;
    let max = *self.iter().max().unwrap_or(&0);

    let y_offset = chart_height / (max) as f32;
    let x_offset = width as f32 / (self.len() as f32 - 1.0);

    let points = self
      .iter()
      .enumerate()
      .map(|(i, p)| (i as f32 * x_offset, chart_height - y_offset * *p as f32))
      .collect::<Vec<(f32, f32)>>();

    let mut d = String::new();
    d.push_str(&format!("M0 {}", points.first().unwrap_or(&(0., 0.)).1));
    for (i, p) in points {
      d.push_str(&format!("L{} {}", i, p));
    }
    BadgeContent {
      content: d,
      width,
      height: chart_height as u32,
    }
  }
}

impl Content for &str {
  fn content(&self, height: u32) -> BadgeContent {
    let font = get_font();

    let scale = Scale::uniform(height as f32);
    let v_metrics = font.v_metrics(scale);

    let normalized = self.trim().nfc().collect::<String>();
    let glyphs: Vec<_> = font.layout(&normalized, scale, point(0., 0.)).collect();

    let glyphs_height = (v_metrics.ascent + v_metrics.descent.abs()).round() as u32;
    let width = {
      let width = glyphs
        .last()
        .map(|g| {
          if let Some(bbox) = g.pixel_bounding_box() {
            bbox.min.x as f32 + g.unpositioned().h_metrics().advance_width
          } else {
            0.0
          }
        })
        .unwrap_or(0.)
        .ceil();
      width as u32 + ((self.len().checked_sub(1).unwrap_or(0)) * 2) as u32
    };
    BadgeContent {
      content: self.to_string(),
      width,
      height: glyphs_height,
    }
  }
}

struct ContentSize {
  x: u32,
  y: u32,
  rw: u32,
}

impl BadgeContent {
  fn content_size(&self, width: u32, padding: u32, height: u32) -> ContentSize {
    let w = self.width + width;
    let x = (self.width + padding) / 2 + width;
    let y = height / 2;
    let mut rw = w;
    rw += match (self.width, width) {
      (x, _) if x > 0 => padding,
      (x, y) if x == 0 && y > 0 => padding / 3 * 2,
      _ => 0,
    };
    ContentSize { x, y, rw }
  }
}

static MULTIPLIER: f32 = 0.65;

impl<'a> Display for Badge<'a, { BadgeTypeState::Init }> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let height = self.height;
    let font_size = (height as f32 * MULTIPLIER).ceil() as u32;
    let padding: u32 = (height as f32 * 0.75) as u32;

    let mut icon_width = 0;
    if let Some(_) = self.icon {
      icon_width = ((height as f32) * MULTIPLIER) as u32;
    }

    let subject = self.subject.content(font_size);
    let subject_size = subject.content_size(icon_width, padding, height);

    let width = subject_size.rw;

    let rx = match (&self.style, subject.height) {
      (Styles::Classic, 30) => 6,
      (Styles::Classic, 40) => 9,
      (Styles::Classic, _) => 3,
      (_, _) => 0,
    };

    let markup = html! {
      svg
        height=(height)
        viewBox={"0 0 " (width) " " (height)}
        width=(width)
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink" {
          @if let Some(icon) = &self.icon { (icon) }
          defs {
            @if &self.style == &Styles::Classic {
              linearGradient id="a" x2="0" y2="100%" {
                stop offset="0" stop-color="#EEE" stop-opacity="0.1" {}
                stop offset="1" stop-opacity="0.1" {}
              }
            }
            mask id="m" {
              rect fill="#fff" height=(height) rx=(rx) width=(width) {}
            }
            filter id="shadow" {
              feDropShadow dx="-0.8" dy="-0.8" stdDeviation="0" flood-color="#000" flood-opacity="0.4" {}
            }
          }
          g#bg mask=@if self.style == Styles::Classic { "url(#m)" } {
            rect fill=@if self.style == Styles::Flat { "#eee" } @else { "url(#a)" } height=(height) width=(width) {}
            rect#subject
              fill=(self.color)
              height=(height)
              width=(width)
              {}
          }
          g#text
            fill="#fff"
            font-family="Verdana,sans-serif"
            font-size=(font_size)
            transform="translate(0, 0)" {
              @if subject.content.len() > 0 {
                text
                  dominant-baseline="central"
                  text-anchor="middle"
                  text-length=(subject.width)
                  x=(subject_size.x)
                  y=(subject_size.y)
                  filter="url(#shadow)"
                  { (subject.content) }
              }
          }
          @if let Some(icon) = &self.icon {
            use
              filter="url(#shadow)"
              xlink:href={"#" (icon.name)}
              x=((padding/3))
              y=(((height  as f32) / 2.0 - (icon_width as f32 / 2.0)))
              width=(icon_width)
              height=(icon_width)
              fill=(icon.color)
              {}
          }
      }
    };
    write!(f, "{}", markup.into_string())
  }
}

impl<'a> Display for Badge<'a, { BadgeTypeState::Text }> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let height = self.height;
    let font_size = (height as f32 * MULTIPLIER).ceil() as u32;
    let padding: u32 = (height as f32 * 0.75) as u32;

    let mut icon_width = 0;
    if let Some(_) = self.icon {
      icon_width = ((height as f32) * MULTIPLIER) as u32;
    }

    let subject = self.subject.content(font_size);
    let subject_size = subject.content_size(icon_width, padding, height);

    let content = self.content.get().unwrap().content(height); //content.get().unwrap().content(height);

    let content_size = ContentSize {
      x: (content.width + padding) / 2,
      y: height / 2,
      rw: content.width + padding,
    };
    let width = subject_size.rw + content_size.rw;
    let rx = match (&self.style, content.height) {
      (Styles::Classic, 30) => 6,
      (Styles::Classic, 40) => 9,
      (Styles::Classic, _) => 3,
      (_, _) => 0,
    };

    let markup = html! {
      svg
        height=(height)
        viewBox={"0 0 " (width) " " (height)}
        width=(width)
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink" {
          @if let Some(icon) = &self.icon { (icon) }
          defs {
            @if &self.style == &Styles::Classic {
              linearGradient id="a" x2="0" y2="100%" {
                stop offset="0" stop-color="#EEE" stop-opacity="0.1" {}
                stop offset="1" stop-opacity="0.1" {}
              }
            }
            mask id="m" {
              rect fill="#fff" height=(height) rx=(rx) width=(width) {}
            }
            filter id="shadow" {
              feDropShadow dx="-0.8" dy="-0.8" stdDeviation="0" flood-color="#000" flood-opacity="0.4" {}
            }
          }
          g#bg mask=@if self.style == Styles::Classic { "url(#m)" } {
            rect fill=@if self.style == Styles::Flat { "#eee" } @else { "url(#a)" } height=(height) width=(width) {}
            rect#subject
              fill="#555"
              height=(height)
              width=(subject_size.rw)
              {}
            rect#content
              fill=(self.color)
              height=(height)
              width=(content_size.rw)
              x=(subject_size.rw)
              {}
          }
          g#text
            fill="#fff"
            font-family="Verdana,sans-serif"
            font-size=(font_size)
            transform="translate(0, 0)" {
              @if subject.content.len() > 0 {
                text
                  dominant-baseline="central"
                  text-anchor="middle"
                  text-length=(subject.width)
                  x=(subject_size.x)
                  y=(subject_size.y)
                  filter="url(#shadow)"
                  { (subject.content) }
              }
              text
                x=((subject_size.rw + content_size.x))
                y=(content_size.y)
                text-length=(content.width)
                text-anchor="middle"
                dominant-baseline="central"
                filter="url(#shadow)"
                { (content.content) }
          }
          @if let Some(icon) = &self.icon {
            use
              filter="url(#shadow)"
              xlink:href={"#" (icon.name)}
              x=((padding/3))
              y=(((height  as f32) / 2.0 - (icon_width as f32 / 2.0)))
              width=(icon_width)
              height=(icon_width)
              fill=(icon.color)
              {}
          }
      }
    };

    write!(f, "{}", markup.into_string())
  }
}

impl<'a> Display for Badge<'a, { BadgeTypeState::Data }> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let height = self.height;

    let font_size = (height as f32 * MULTIPLIER).ceil() as u32;
    let padding: u32 = (height as f32 * 0.75) as u32;

    let mut icon_width = 0;
    if let Some(_) = self.icon {
      icon_width = ((height as f32) * MULTIPLIER) as u32;
    }

    let subject = self.subject.content(font_size);
    let subject_size = subject.content_size(icon_width, padding, height);

    let content = &self.content.get().unwrap().content(height); //content.get().unwrap().content(height);

    let content_size = match self.style {
      Styles::Flat => ContentSize {
        x: (content.width + padding) / 2,
        y: content.height / 2,
        rw: content.width,
      },
      _ => ContentSize {
        x: (content.width + padding) / 2,
        y: content.height / 2,
        rw: content.width + 5,
      },
    };

    let width = subject_size.rw + content_size.rw;

    let rx = match (&self.style, content.height) {
      (Styles::Classic, 30) => 6,
      (Styles::Classic, 40) => 9,
      (Styles::Classic, _) => 3,
      (_, _) => 0,
    };

    let markup = html! {
      svg
        height=(height)
        viewBox={"0 0 " (width) " " (height)}
        width=(width)
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink" {
          @if let Some(icon) = &self.icon { (icon) }
          defs {
            @if &self.style == &Styles::Classic {
              linearGradient id="a" x2="0" y2="100%" {
                stop offset="0" stop-color="#EEE" stop-opacity="0.1" {}
                stop offset="1" stop-opacity="0.1" {}
              }
            }
            mask id="m" {
              rect fill="#fff" height=(height) rx=(rx) width=(width) {}
            }
            filter id="shadow" {
              feDropShadow dx="-0.8" dy="-0.8" stdDeviation="0" flood-color="#000" flood-opacity="0.4" {}
            }
          }
          g#bg mask=@if self.style == Styles::Classic { "url(#m)" } {
            rect fill=@if self.style == Styles::Flat { "#eee" } @else { "url(#a)" } height=(height) width=(width) {}
            rect#subject
              fill="#555"
              height=(height)
              width=(subject_size.rw)
              {}
            rect#content
              fill="#eee"
              height=(height)
              width=(content_size.rw)
              x=(subject_size.x)
              {}
          }
          g#text
            fill="#fff"
            font-family="Verdana,sans-serif"
            font-size=(font_size)
            transform="translate(0, 0)" {
              @if subject.content.len() > 0 {
                text
                  dominant-baseline="central"
                  text-anchor="middle"
                  text-length=(subject.width)
                  x=(subject_size.x)
                  y=(subject_size.y)
                  filter="url(#shadow)"
                  { (subject.content) }
              }
              path
                fill="none"
                transform=(format!("translate({}, {})", subject_size.rw, 0))
                stroke=(self.color)
                stroke-width="1px"
                d=(content.content)
                {}
              path
                fill=(self.color)
                fill-opacity="0.2"
                transform=(format!("translate({}, {})", subject_size.rw, 0))
                stroke="none"
                stroke-width="0px"
                d=(format!("{}V{}H0Z", content.content, height))
                {}
          }
          @if let Some(icon) = &self.icon {
            use
              filter="url(#shadow)"
              xlink:href={"#" (icon.name)}
              x=((padding/3))
              y=(((height  as f32) / 2.0 - (icon_width as f32 / 2.0)))
              width=(icon_width)
              height=(icon_width)
              fill=(icon.color)
              {}
          }
      }
    };

    write!(f, "{}", markup.into_string())
  }
}

#[cfg(test)]
mod tests {
  use super::{get_color, Badge, Content, Size, Styles};
  use scraper::{Html, Selector};

  use crate::Icon;

  const DEF_COLOUR: &str = "#08C";
  #[test]
  fn default_badge_has_classic_style() {
    let badge = Badge::new("just text");
    let badge_svg = badge.to_string();
    let doc = Html::parse_fragment(&badge_svg);
    assert_eq!(badge.style, Styles::Classic, "style not Classic");
    let lg_selector = Selector::parse("linearGradient").unwrap();
    assert!(doc.select(&lg_selector).next().is_some());
  }
  #[test]
  fn default_badge_has_20px_height() {
    let badge = Badge::new("just text");
    let badge_svg = badge.to_string();
    let doc = Html::parse_fragment(&badge_svg);
    let selector = Selector::parse("svg").unwrap();
    let svg = doc.select(&selector).next().unwrap();
    assert_eq!(svg.value().attr("height"), Some("20"));
  }
  #[test]
  fn default_badge_only_has_subject() {
    let badge = Badge::new("just subject");
    let badge_svg = badge.to_string();
    let doc = Html::parse_fragment(&badge_svg);
    let text_sel = Selector::parse("g#text > text").unwrap();
    let text_els = doc.select(&text_sel);
    assert_eq!(text_els.count(), 1);
    let text = doc.select(&text_sel).next().unwrap();
    assert_eq!(text.text().collect::<String>(), String::from("just subject"));
  }
  #[test]
  fn default_badge_has_333_as_background_colour() {
    let mut badge = Badge::new("just text");
    badge.color(DEF_COLOUR);
    let def_color = get_color(DEF_COLOUR).unwrap();
    let badge_svg = badge.to_string();
    let doc = Html::parse_fragment(&badge_svg);
    let rect_sel = Selector::parse("g#bg > rect#subject").unwrap();
    let rect = doc.select(&rect_sel).next().unwrap();
    assert_eq!(rect.value().attr("fill").unwrap(), &def_color);
  }

  #[test]
  fn badge_with_text() {
    let badge = Badge::new("with subject").text("badge text");
    // let content = &badge.content;
    // assert_eq!(content.get(), Some("badge text"));
    let doc = Html::parse_fragment(&badge.to_string());
    let subject_sel = Selector::parse("g#text > text:last-child").unwrap();
    let subject = doc.select(&subject_sel).next().unwrap();
    assert_eq!(subject.text().collect::<String>(), String::from("badge text"));
  }

  #[test]
  fn badge_with_icon() {
    let icon = Icon::new("git").build().unwrap();
    let mut badge = Badge::new("with icon");
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
    let mut badge = Badge::new("with icon");
    badge.size(Size::Medium);
    let doc = Html::parse_fragment(&badge.to_string());
    let svg_sel = Selector::parse("svg").unwrap();
    let svg = doc.select(&svg_sel).next().unwrap();
    assert_eq!(svg.value().attr("height"), Some("30"));
  }
  #[test]
  fn badge_has_large_icon() {
    let mut badge = Badge::new("with icon");
    badge.size(Size::Large);
    let doc = Html::parse_fragment(&badge.to_string());
    let svg_sel = Selector::parse("svg").unwrap();
    let svg = doc.select(&svg_sel).next().unwrap();
    assert_eq!(svg.value().attr("height"), Some("40"));
  }

  #[test]
  fn badge_with_data() {
    let badge = Badge::new("Some data").data(vec![1, 2, 3, 4, 5]);

    let doc = Html::parse_fragment(&badge.to_string());
    println!("{:?}", &badge.to_string());
    let line_sel = Selector::parse("path").unwrap();
    let svg = doc.select(&line_sel).next().unwrap();
    assert!(svg.value().attr("d").is_some());
  }

  #[test]
  fn content_text_has_width() {
    let text = "".content(20);
    assert_eq!(text.width, 0);
    let text = "npm".content(20);
    assert_eq!(text.width, 43);
    let text = "long text".content(20);
    assert_eq!(text.width, 90);
  }

  #[test]
  fn content_data_has_width() {
    let d1 = vec![].content(20);
    assert_eq!(d1.width, 0);
    let d2 = vec![2, 4, 3, 2].content(20);
    assert_eq!(d2.width, 100);
  }

  #[test]
  fn content_data_is_same() {
    let d1 = vec![2, 4, 3, 2].content(20);
    let d2 = &vec![2, 4, 3, 2].content(20);
    assert_eq!(d1.content, d2.content);
  }
}
