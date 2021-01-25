use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use unicode_normalization::UnicodeNormalization;

fn get_font() -> FontRef<'static> {
  let font_data: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resx/Verdana.ttf"));
  FontRef::try_from_slice(font_data).expect("Error constructing Font")
}

pub(crate) fn get_text_width(text: &str, height: f32) -> usize {
  let font = get_font();

  let scale = PxScale::from(height as f32);
  let scaled_font = font.as_scaled(scale);

  let normalized = text.trim().nfc().collect::<String>();

  normalized
    .chars()
    .map(|c| {
      let glyph = scaled_font.scaled_glyph(c);
      let gb = scaled_font.glyph_bounds(&glyph);
      gb.width() * 1.12
    })
    .fold(0.0, |acc, w| acc + w) as usize
}

#[derive(Debug, Default)]
pub struct Path<'a> {
  values: &'a [f32],
  chart_height: f32,
  x_offset: f32,
  y_offset: f32,
  index: usize,
}

impl<'a> Path<'a> {
  pub fn new(values: &'a [f32], height: usize, width: usize) -> Self {
    let chart_height = height as f32;
    let max = *values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.);

    let y_offset = chart_height / max;
    let x_offset = width as f32 / (values.len() as f32 - 1.0);

    Path {
      values,
      chart_height,
      x_offset,
      y_offset,
      index: 0,
    }
  }
}

impl<'a> Iterator for Path<'a> {
  type Item = (f32, f32);
  fn next(&mut self) -> Option<Self::Item> {
    let index = self.index;
    if index >= self.values.len() {
      return None;
    }
    let p = self.values[index];
    let x = index as f32 * self.x_offset;
    let y = self.chart_height - self.y_offset * p as f32;
    self.index += 1;
    Some((x, y))
  }
}

#[derive(Default)]
pub(super) struct ContentSize {
  pub(super) x: usize,
  pub(super) y: usize,
  pub(super) rw: usize,
}

pub(super) fn content_size(
  width: usize,
  icon_width: usize,
  padding: usize,
  height: usize,
  x_offset: usize,
) -> ContentSize {
  let w = width + icon_width + x_offset;
  let x = (width + padding) / 2 + icon_width + x_offset;
  let y = height / 2;
  let mut rw = w;
  rw += match (width, icon_width) {
    (x, _) if x > 0 => padding,
    (0, y) if y > 0 => padding / 3 * 2,
    _ => 0,
  };
  ContentSize { x, y, rw }
}

#[cfg(test)]
mod tests {
  use super::{get_text_width, Path};

  #[test]
  fn content_str_width() {
    let s = "Hello";
    let bc = get_text_width(s, 20.);
    assert!(bc > 0);
  }
  #[test]
  fn content_text_has_width() {
    let text = get_text_width("", 20.);
    assert_eq!(text, 0);
    let text = get_text_width("npm", 20.);
    assert_eq!(text, 46);
    let text = get_text_width("long text", 20.);
    assert_eq!(text, 90);
  }

  #[test]
  fn path_generate() {
    let d = [2., 4., 3., 2.];
    let path = Path::new(&d, 20, 100).into_iter().collect::<Vec<_>>();

    assert_eq!(path.len(), 4);
    assert_eq!(
      path,
      vec![(0.0, 10.0), (33.333332, 0.0), (66.666664, 5.0), (100.0, 10.0)]
    )
  }
}
