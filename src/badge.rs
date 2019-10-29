use super::{get_color, icons::Icon};
use maud::{html, PreEscaped};
use rusttype::{point, Font, FontCollection, Scale};
use std::fmt;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, PartialEq)]
pub enum Styles {
  Flat,
  Classic,
}

#[derive(Debug, PartialEq)]
pub enum Size {
  Large,
  Medium,
  Small,
}

#[derive(Debug)]
pub struct Badge<'a> {
  subject: &'a str,
  text: Option<&'a str>,
  color: String,
  style: Styles,
  icon: Option<Icon<'a>>,
  height: u32,
  data: Option<Vec<i64>>,
}

impl<'a> Badge<'a> {
  pub fn new(subject: &'a str) -> Self {
    Badge {
      subject: subject,
      text: None,
      color: "#08C".into(),
      style: Styles::Classic,
      icon: None,
      height: 20,
      data: None,
    }
  }
  pub fn color(&mut self, color: &'a str) -> &mut Self {
    if let Some(c) = get_color(color) {
      self.color = c;
    }
    self
  }
  pub fn text(&mut self, text: &'a str) -> &mut Self {
    self.text = Some(text);
    self
  }
  pub fn style(&mut self, style: Styles) -> &mut Self {
    self.style = style;
    self
  }
  pub fn icon(&mut self, icon: Option<Icon<'a>>) -> &mut Self {
    self.icon = icon;
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
  pub fn data(&mut self, data: Vec<i64>) -> &mut Self {
    self.data = Some(data);
    self
  }
  fn to_svg(&self) -> String {
    let font = get_font();
    let height = self.height;
    let font_size = (height as f32 * 0.65).ceil() as u32;
    let padding: u32 = (height as f32 * 0.75) as u32;

    let mut icon_width = 0;
    if let Some(_) = &self.icon {
      icon_width = ((self.height as f32) * 0.65) as u32;
    }

    let subject = Content::with_text(&self.subject, font_size, &font).unwrap();
    let subject_size = {
      let w = subject.width + icon_width;
      let x = (subject.width + padding) / 2 + icon_width;
      let y = height / 2;
      let mut rw = w;
      rw += match (subject.width, icon_width) {
        (x, _) if x > 0 => padding,
        (x, y) if x == 0 && y > 0 => padding / 3 * 2,
        _ => 0,
      };
      (x, y, rw)
    };
    let content = match (&self.data, &self.text) {
      (Some(c), _) => Content::with_data(c, height),
      (_, Some(t)) => Content::with_text(t, font_size, &font),
      (_, _) => None,
    };

    let content_size = match (&content, &self.style) {
      (Some(c), Styles::Flat) if c.is_data => ((c.width + padding) / 2, height / 2, c.width),
      (Some(c), _) if c.is_data => ((c.width + padding) / 2, height / 2, c.width + 5),
      (Some(c), _) => ((c.width + padding) / 2, height / 2, c.width + padding),
      (_, _) => (0, 0, 0),
    };

    let width = subject_size.2 + content_size.2;
    let rx = match self.height {
      30 => 6,
      40 => 9,
      _ => 3,
    };

    let markup = html! {
      svg
        height=(height)
        viewBox={"0 0 " (width) " " (height)}
        width=(width)
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        {
          defs {
            @if self.style == Styles::Classic {
              linearGradient id="a" x2="0" y2="100%" {
                stop offset="0" stop-color="#EEE" stop-opacity="0.1" {}
                stop offset="1" stop-opacity="0.1" {}
              }
              mask id="m" {
                rect fill="#fff" height=(height) rx=(rx) width=(width) {}
              }
            }
            filter id="shadow" {
              feDropShadow dx="-0.8" dy="-0.8" stdDeviation="0" flood-color="#000" flood-opacity="0.4" {}
            }
            @if let Some(icon) = &self.icon {
              (PreEscaped(icon.symbol.to_owned()))
            }
          }
          g#bg mask=@if self.style == Styles::Classic { "url(#m)" } {
            @match (&self.style, &content) {
              (Styles::Flat, Some(c)) if c.is_data => {
                rect fill="#eee" height=(height) width=(width) {}
              },
              (_, _) => rect fill="url(#a)" height=(height) width=(width) {},
            }
            rect#subject
              fill=@if content.is_some() { "#555" } @else { (self.color) }
              height=(height)
              width=(subject_size.2)
              {}
            rect#content
              fill=@match &content {
                Some(c) if c.is_data => "#eee",
                _ => (self.color)
              }
              height=(height)
              width=(content_size.2)
              x=(subject_size.2)
              {}
          }
        g#text
          fill="#fff"
          font-family="Verdana,sans-serif"
          font-size=(font_size)
          transform="translate(0, 0)"
          {
            @if subject.content.len() > 0 {
              text
                dominant-baseline="central"
                text-anchor="middle"
                text-length=(subject.width)
                x=(subject_size.0)
                y=(subject_size.1)
                filter="url(#shadow)"
                { (subject.content) }
            }
            @match &content {
              Some(c) if c.is_data => {
                path
                  fill="none"
                  transform=(format!("translate({}, {})", subject_size.2, 0))
                  stroke=(self.color)
                  stroke-width="1px"
                  d=(c.content)
                  {}
                path
                  fill=(self.color)
                  fill-opacity="0.2"
                  transform=(format!("translate({}, {})", subject_size.2, 0))
                  stroke="none"
                  stroke-width="0px"
                  d=(format!("{}V{}H0Z", c.content, height))
                  {}
              },
              Some(c) => {
                text
                  x=((subject_size.2 + content_size.0))
                  y=(content_size.1)
                  text-length=(c.width)
                  text-anchor="middle"
                  dominant-baseline="central"
                  filter="url(#shadow)"
                  { (c.content) }
              },
              _ => {}
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
    markup.into_string()
  }
}

impl<'a> fmt::Display for Badge<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_svg())
  }
}

fn get_font() -> Font<'static> {
  let font_data: &[u8] = include_bytes!("./resx/Verdana.ttf");
  let font_col = FontCollection::from_bytes(font_data).expect("Error constructing Font");
  font_col.into_font().unwrap()
}

#[derive(Debug)]
struct Content {
  content: String,
  width: u32,
  height: u32,
  is_data: bool,
}
impl Content {
  fn with_data(data: &Vec<i64>, height: u32) -> Option<Self> {
    let width = height * 5;
    let chart_height = (height - 2) as f32;
    let max = data.iter().max()?;
    let min = data.iter().min()?;
    let s = chart_height / (max - min) as f32;

    let offset = width as f32 / (data.len() as f32 - 1.0);

    let mut d = String::new();
    let first = data.first()?;
    d.push_str(&format!("M0 {}", chart_height - (s * (first - min) as f32)));
    for (i, p) in data.iter().enumerate() {
      d.push_str(&format!(
        "L{} {}",
        i as f32 * offset,
        chart_height - (s * (p - min) as f32)
      ));
    }
    Some(Content {
      content: d,
      width: width,
      height: chart_height as u32,
      is_data: true,
    })
  }
  fn with_text(text: &str, height: u32, font: &Font) -> Option<Self> {
    let scale = Scale::uniform(height as f32);
    let v_metrics = font.v_metrics(scale);
    if text.is_empty() {
      return None;
    }
    let normalized = text.trim().nfc().collect::<String>();
    let glyphs: Vec<_> = font.layout(&normalized, scale, point(0.0, 0.0)).collect();

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
        .unwrap()
        .ceil() as u32;
      width + ((text.len() as u32 - 1) * 2)
    };
    Some(Content {
      content: text.to_owned(),
      width,
      height: glyphs_height,
      is_data: false,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::{get_color, get_font, Badge, Content, Size, Styles};
  use scraper::{Html, Selector};

  use crate::IconBuilder;

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
    assert_eq!(
      text.text().collect::<String>(),
      String::from("just subject")
    );
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
    let mut badge = Badge::new("with subject");
    badge.text("badge text");
    let doc = Html::parse_fragment(&badge.to_string());
    assert_eq!(badge.text, Some("badge text"));
    let subject_sel = Selector::parse("g#text > text:last-child").unwrap();
    let subject = doc.select(&subject_sel).next().unwrap();
    assert_eq!(
      subject.text().collect::<String>(),
      String::from("badge text")
    );
  }

  #[test]
  fn badge_with_icon() {
    let mut badge = Badge::new("with icon");
    let icon = IconBuilder::new("git").build();
    badge.icon(icon);
    let doc = Html::parse_fragment(&badge.to_string());
    assert!(badge.icon.is_some());
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
    let mut badge = Badge::new("Some data");
    badge.data(vec![1, 2, 3, 4, 5]);

    let doc = Html::parse_fragment(&badge.to_string());
    let line_sel = Selector::parse("path").unwrap();
    let svg = doc.select(&line_sel).next().unwrap();
    assert!(svg.value().attr("d").is_some());
  }
  #[test]
  fn content_text_has_width() {
    let font = get_font();
    let text = Content::with_text("", 20, &font);
    assert!(text.is_none());
    let text = Content::with_text("npm", 20, &font).unwrap();
    assert_eq!(text.width, 43);
    let text = Content::with_text("long text", 20, &font).unwrap();
    assert_eq!(text.width, 90);
  }
  #[test]
  fn content_data_is_same() {
    let d1 = Content::with_data(&vec![2, 4, 3, 2], 20).unwrap();
    let d2 = Content::with_data(&vec![2, 4, 3, 2], 20).unwrap();
    assert_eq!(d1.content, d2.content);
  }
}
