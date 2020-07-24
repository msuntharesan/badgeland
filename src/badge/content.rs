use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use unicode_normalization::UnicodeNormalization;

fn get_font() -> FontRef<'static> {
  let font_data: &[u8] = include_bytes!("../resx/Verdana.ttf");
  FontRef::try_from_slice(font_data).expect("Error constructing Font")
}

#[derive(Debug, Default)]
pub(super) struct BadgeContent {
  pub(super) content: String,
  pub(super) width: usize,
  pub(super) height: usize,
}

pub(super) trait Content {
  fn content(&self, height: usize) -> BadgeContent;
}

impl<'a> Content for Vec<i64> {
  fn content(&self, height: usize) -> BadgeContent {
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
      height: chart_height as usize,
    }
  }
}

impl Content for &str {
  fn content(&self, height: usize) -> BadgeContent {
    let font = get_font();

    let scale = PxScale::from(height as f32);
    let scaled_font = font.as_scaled(scale);

    let normalized = self.trim().nfc().collect::<String>();

    let glyphs_height = scaled_font.height().ceil() as usize;
    let width = normalized
      .chars()
      .scan(None, |prev_glyph, c| {
        let mut x = 0.0;
        let glyph = scaled_font.scaled_glyph(c);

        if let Some(last) = prev_glyph.take() {
          x += scaled_font.kern(last, glyph.id);
        }

        x += scaled_font.h_advance(glyph.id);
        *prev_glyph = Some(glyph.id);
        Some(x)
      })
      .fold(0.0, |acc, x| acc + x);

    BadgeContent {
      content: self.to_string(),
      width: width as usize,
      height: glyphs_height,
    }
  }
}

pub(super) struct ContentSize {
  pub(super) x: usize,
  pub(super) y: usize,
  pub(super) rw: usize,
}

impl BadgeContent {
  pub(super) fn content_size(&self, width: usize, padding: usize, height: usize, x_offset: usize) -> ContentSize {
    let w = self.width + width + x_offset;
    let x = (self.width + padding) / 2 + width + x_offset;
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

#[cfg(test)]
mod tests {
  use super::Content;

  #[test]
  fn content_str_width() {
    let s = "Hello";
    let bc = s.content(20);
    assert!(bc.width > 0);
  }
}
