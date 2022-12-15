use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use once_cell::sync::Lazy;
use unicode_normalization::UnicodeNormalization;

static FONT: Lazy<FontRef<'static>> = Lazy::new(|| {
    let font_data: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resx/Verdana.ttf"));
    FontRef::try_from_slice(font_data).expect("Error constructing Font")
});

pub(crate) trait TextWidth {
    fn text_width(&self, height: f32) -> usize;
}

impl<'a> TextWidth for &'a str {
    #[inline]
    fn text_width(&self, height: f32) -> usize {
        let scale = PxScale::from(height);
        let scaled_font = FONT.as_scaled(scale);

        let normalized: String = self.trim().nfc().collect();
        normalized
            .chars()
            .map(|c| {
                let glyph = scaled_font.scaled_glyph(c);
                let gb = scaled_font.glyph_bounds(&glyph);
                gb.width() * 1.12
            })
            .fold(0.0, |acc, w| acc + w)
            .floor() as usize
    }
}

pub(super) trait SvgPath {
    fn svg_path(&self, height: usize, width: usize) -> String;
}

impl<'a> SvgPath for [f32] {
    fn svg_path(&self, height: usize, width: usize) -> String {
        let len = self.len();
        let chart_height = height as f32;
        let max = *self.iter().max_by_key(|&a| *a as i32).unwrap_or(&0.);

        let y_offset = chart_height / max;
        let x_offset = width as f32 / (len as f32 - 1.0);

        let mut path_str = String::new();

        for (i, v) in self.iter().enumerate() {
            let x = i as f32 * x_offset;
            let y = chart_height - y_offset * v;
            let path = match i {
                0 => format!("M0 {y}L{x} {y}", x = 0, y = y),
                _ => format!("L{x} {y}", x = x, y = y),
            };
            path_str.push_str(&path);
        }
        path_str
    }
}

#[derive(Default)]
pub(super) struct ContentSize {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) rw: usize,
}

pub(super) trait BadgeContentSize {
    fn content_size(
        &self,
        height: usize,
        width: usize,
        padding: usize,
        x_offset: usize,
    ) -> ContentSize;
}

impl<'a> BadgeContentSize for &'a [f32] {
    #[inline]
    fn content_size(&self, height: usize, width: usize, padding: usize, _: usize) -> ContentSize {
        ContentSize {
            x: (width + padding) / 2,
            y: height / 2,
            rw: width,
        }
    }
}

impl<'a> BadgeContentSize for &'a str {
    #[inline]
    fn content_size(
        &self,
        height: usize,
        width: usize,
        padding: usize,
        x_offset: usize,
    ) -> ContentSize {
        let w = width + x_offset;
        let x = (width + padding) / 2 + x_offset;
        let y = height / 2;
        let rw = w + padding;
        ContentSize { x, y, rw }
    }
}

#[cfg(test)]
mod tests {
    use super::{SvgPath, TextWidth};

    #[test]
    fn content_str_width() {
        let s = "Hello";
        let bc = s.text_width(20.);
        assert!(bc > 0);
    }
    #[test]
    fn content_text_has_width() {
        let text = "".text_width(20.);
        assert_eq!(text, 0);
        let text = "npm".text_width(20.);
        assert_eq!(text, 46);
        let text = "long text".text_width(20.);
        assert_eq!(text, 90);
    }

    #[test]
    fn path_generate() {
        let d: &[f32; 4] = &[2., 4., 3., 2.];
        let path = &d.svg_path(20, 100);

        assert_eq!(path, "M0 10L0 10L33.333332 0L66.666664 5L100 10")
    }
}
