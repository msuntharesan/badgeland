use super::Styles;
use rusttype::{point, Font, Scale};
use unicode_normalization::UnicodeNormalization;

fn get_font() -> Font<'static> {
  let font_data: &[u8] = include_bytes!("../resx/Verdana.ttf");
  Font::try_from_bytes(font_data).expect("Error constructing Font")
}

#[derive(Debug, Default)]
pub(super) struct BadgeContent {
  pub(super) content: String,
  pub(super) width: u32,
  pub(super) height: u32,
}

pub(super) trait Content {
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

pub(super) struct ContentSize {
  pub(super) x: u32,
  pub(super) y: u32,
  pub(super) rw: u32,
}

impl BadgeContent {
  pub(super) fn content_size(&self, width: u32, padding: u32, height: u32) -> ContentSize {
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
  pub(super) fn rx(&self, style: &Styles) -> usize {
    match (style, self.height) {
      (Styles::Classic, 40) => 9,
      (Styles::Classic, 30) => 6,
      (Styles::Classic, _) => 3,
      (_, _) => 0,
    }
  }
}
