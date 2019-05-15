extern crate maud;
extern crate rusttype;
extern crate unicode_normalization;


use super::{get_color, icons::Icon};
use maud::{html, PreEscaped};
use rusttype::{point, Font, FontCollection, Scale};
use std::borrow::Cow;
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
  subject: Option<&'a str>,
  text: &'a str,
  color: Cow<'a, str>,
  style: Styles,
  icon: Option<Icon<'a>>,
  height: u32,
}

impl<'a> Badge<'a> {
  pub fn new(text: &'a str) -> Self {
    Badge {
      text: text,
      color: "#08C".into(),
      subject: None,
      style: Styles::Classic,
      icon: None,
      height: 20,
    }
  }
  pub fn color<S>(&mut self, color: S) -> &mut Self
  where
    S: Into<Cow<'a, str>>,
  {
    let c = color.into();
    let c = get_color(&c);
    if let Some(c) = c {
      self.color = c.into();
    }
    self
  }
  pub fn subject(&mut self, subject: &'a str) -> &mut Self {
    self.subject = Some(subject);
    self
  }
  pub fn style(&mut self, style: Styles) -> &mut Self {
    self.style = style;
    self
  }

  pub fn icon(&mut self, icon_key: &'a str, color: Option<&'a str>) -> &mut Self {
    if let Some(mut icon) = Icon::new(icon_key) {
      icon.size(((self.height as f32) * 0.65) as u32);
      if let Some(c) = color {
        icon.color(c);
      }
      self.icon = Some(icon);
    }
    self
  }
  pub fn size(&mut self, size: Size) -> &mut Self {
    let height: u32 = match size {
      Size::Small => 20,
      Size::Medium => 30,
      Size::Large => 40,
    };
    self.height = height;
    if let Some(icon) = &mut self.icon {
      icon.size((height as f32 * 0.65) as u32);
    }
    self
  }
  pub fn to_svg(self: &Self) -> String {
    let font = get_font();
    let height = self.height;
    let font_size = (height as f32 * 0.65).ceil() as u32;
    let padding: u32 = (height as f32 * 0.75) as u32;
    let subject = TextData::new(self.subject.unwrap_or_default(), font_size, &font);
    let text = TextData::new(self.text, font_size, &font);

    let mut icon_width = 0;
    if let Some(icon) = &self.icon {
      icon_width = icon.size;
    }

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
    let text_size = {
      let x = (text.width + padding) / 2;
      let y = height / 2;
      let rw = text.width + padding;
      (x, y, rw)
    };
    let width = subject_size.2 + text_size.2;
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
              feDropShadow dx="-1" dy="-1" stdDeviation="0" flood-color="#000" flood-opacity="0.5" {}
            }
            @if let Some(icon) = &self.icon {
              (PreEscaped(icon.symbol))
            }
          }
          g#bg mask=@if self.style == Styles::Classic { "url(#m)" } {
            @if subject_size.2 > 0 {
              rect#subject fill="#555" height=(height) width=(subject_size.2) {}
            }
            rect#text fill=(self.color) height=(height) width=(text_size.2) x=(subject_size.2) {}
            @if self.style == Styles::Classic {
              rect fill="url(#a)" height=(height) width=(width) {}
            }
          }
        g#text fill="#fff" font-family="Verdana,sans-serif" font-size=(font_size) {
          @if subject.text.len() > 0{
            text
              dominant-baseline="central"
              text-anchor="middle"
              text-length=(subject.width)
              x=(subject_size.0)
              y=(subject_size.1)
              filter="url(#shadow)"
              { (subject.text) }
          }
          text
            x=((subject_size.2 + text_size.0))
            y=(text_size.1)
            text-length=(text.width)
            text-anchor="middle"
            dominant-baseline="central"
            filter="url(#shadow)"
            { (text.text) }
        }
        @if let Some(icon) = &self.icon {
          use
            filter="url(#shadow)"
            xlink:href={"#" (icon.name)}
            x=((padding/3))
            y=(((height  as f32) / 2.0 - (icon_width as f32 / 2.0)))
            width=(icon.size)
            height=(icon.size)
            fill=(icon.color)
            {}
        }
      }
    };
    markup.into_string()
  }
}

fn get_font() -> Font<'static> {
  let font_data: &[u8] = include_bytes!("../resx/Verdana.ttf");
  let font_col = FontCollection::from_bytes(font_data).expect("Error constructing Font");
  font_col.into_font().unwrap()
}

#[derive(Debug)]
struct TextData<'a> {
  text: &'a str,
  width: u32,
  height: u32,
}
impl<'a> TextData<'a> {
  fn new(text: &'a str, height: u32, font: &'a Font<'a>) -> Self {
    let scale = Scale::uniform(height as f32);
    let v_metrics = font.v_metrics(scale);
    if text.is_empty() {
      TextData {
        text: text,
        width: 0,
        height: 0,
      }
    } else {
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

      TextData {
        text: text,
        width: width,
        height: glyphs_height,
      }
    }
  }
}


#[cfg(test)]
mod tests {
  use super::{get_font, Badge, Size, Styles, TextData};
  use scraper::{Html, Selector};

  const DEF_COLOUR: &str = "#08C";
  #[test]
  fn default_badge_has_classic_style() {
    let badge = Badge::new("just text");
    let badge_svg = badge.to_svg();
    let doc = Html::parse_fragment(&badge_svg);
    assert_eq!(badge.style, Styles::Classic, "style not Classic");
    let lg_selector = Selector::parse("linearGradient").unwrap();
    assert!(doc.select(&lg_selector).next().is_some());
  }
  #[test]
  fn default_badge_has_20px_height() {
    let badge = Badge::new("just text");
    let badge_svg = badge.to_svg();
    let doc = Html::parse_fragment(&badge_svg);
    let selector = Selector::parse("svg").unwrap();
    let svg = doc.select(&selector).next().unwrap();
    assert_eq!(svg.value().attr("height"), Some("20"));
  }
  #[test]
  fn default_badge_only_has_text() {
    let badge = Badge::new("just text");
    let badge_svg = badge.to_svg();
    let doc = Html::parse_fragment(&badge_svg);
    let text_sel = Selector::parse("g#text > text").unwrap();
    let text_els = doc.select(&text_sel);
    assert_eq!(text_els.count(), 1);
    let text = doc.select(&text_sel).next().unwrap();
    assert_eq!(text.text().collect::<String>(), String::from("just text"));
  }
  #[test]
  fn default_badge_has_333_as_background_colour() {
    let mut badge = Badge::new("just text");
    badge.color(DEF_COLOUR);
    let badge_svg = badge.to_svg();
    let doc = Html::parse_fragment(&badge_svg);
    let rect_sel = Selector::parse("g#bg > rect#text").unwrap();
    let rect = doc.select(&rect_sel).next().unwrap();
    assert_eq!(rect.value().attr("fill"), Some(DEF_COLOUR));
  }

  #[test]
  fn badge_with_subject() {
    let mut badge = Badge::new("with subject");
    badge.subject("badge subject");
    let doc = Html::parse_fragment(&badge.to_svg());
    assert_eq!(badge.subject, Some("badge subject"));
    let subject_sel = Selector::parse("g#text > text:first-child").unwrap();
    let subject = doc.select(&subject_sel).next().unwrap();
    assert_eq!(
      subject.text().collect::<String>(),
      String::from("badge subject")
    );
  }

  #[test]
  fn badge_with_icon() {
    let mut badge = Badge::new("with icon");
    badge.icon("git", None);
    let doc = Html::parse_fragment(&badge.to_svg());
    assert!(badge.icon.is_some());
    let icon_sel = Selector::parse("symbol").unwrap();
    let icon_symbol = doc.select(&icon_sel).next().unwrap();
    assert_eq!(icon_symbol.value().attr("id"), Some("git"));
  }
  #[test]
  fn badge_has_medium_icon() {
    let mut badge = Badge::new("with icon");
    badge.size(Size::Medium);
    let doc = Html::parse_fragment(&badge.to_svg());
    let svg_sel = Selector::parse("svg").unwrap();
    let svg = doc.select(&svg_sel).next().unwrap();
    assert_eq!(svg.value().attr("height"), Some("30"));
  }
  #[test]
  fn badge_has_large_icon() {
    let mut badge = Badge::new("with icon");
    badge.size(Size::Large);
    let doc = Html::parse_fragment(&badge.to_svg());
    let svg_sel = Selector::parse("svg").unwrap();
    let svg = doc.select(&svg_sel).next().unwrap();
    assert_eq!(svg.value().attr("height"), Some("40"));
  }
  #[test]
  fn text_data_has_width() {
    let font = get_font();
    let text = TextData::new("", 20, &font);
    assert_eq!(text.width, 0);
    let text = TextData::new("npm", 20, &font);
    assert_eq!(text.width, 43);
    let text = TextData::new("long text", 20, &font);
    assert_eq!(text.width, 90);
  }
  #[test]
  fn text_data_has_same_width() {
    let font = get_font();
    let text1 = TextData::new("Á", 20, &font);
    let text2 = TextData::new("Â", 20, &font);
    assert_eq!(text1.width, text2.width);
  }
}
