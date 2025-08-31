use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::RwLock;
use unicode_normalization::UnicodeNormalization;

static FONT: Lazy<FontRef<'static>> = Lazy::new(|| {
    let font_data: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resx/Verdana.ttf"));
    FontRef::try_from_slice(font_data).expect("Error constructing Font")
});

// Precomputed ASCII glyph widths at PxScale = 1.0.
// For ASCII-only text, we can sum these widths and scale by the requested
// height to avoid per-call glyph measurement.
static ASCII_WIDTHS_1_0: Lazy<[f32; 128]> = Lazy::new(|| {
    let scaled_font = FONT.as_scaled(PxScale::from(1.0));
    let mut arr = [0.0f32; 128];
    for b in 0u8..=127u8 {
        let c = b as char;
        let glyph = scaled_font.scaled_glyph(c);
        // Use bounds width for parity with previous behavior
        let gb = scaled_font.glyph_bounds(&glyph);
        arr[b as usize] = gb.width();
    }
    arr
});

// Cache for non-ASCII glyph widths keyed by (codepoint, scaled-height-key)
// to avoid repeated glyph bounds calculations for common unicode characters.
// The height key quantizes the font size by 0.1 to a u32 to avoid f32 hash keys.
static UNICODE_WIDTH_CACHE: Lazy<RwLock<HashMap<(u32, u32), f32>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub(crate) trait TextWidth {
    fn text_width(&self, height: f32) -> usize;
}

impl<'a> TextWidth for &'a str {
    #[inline]
    fn text_width(&self, height: f32) -> usize {
        let s = self.trim();
        if s.is_empty() {
            return 0;
        }

        // Fast path for ASCII-only strings: use cached widths at scale 1.0,
        // scale them by the requested height, and apply the 1.12 factor to
        // preserve the previous sizing behavior.
        if s.is_ascii() {
            let sum_1_0: f32 = s.as_bytes().iter().map(|&b| ASCII_WIDTHS_1_0[b as usize]).sum();
            return (sum_1_0 * height * 1.12).floor() as usize;
        }

        // General path without allocation: iterate NFC chars directly.
        let scale = PxScale::from(height);
        let scaled_font = FONT.as_scaled(scale);

        // Quantize height to 0.1 precision for cache key
        let height_key = (height * 10.0).round() as u32;

        let mut total = 0.0f32;
        for c in s.nfc() {
            let key = (c as u32, height_key);
            // Fast read-lock path
            if let Some(&w) = UNICODE_WIDTH_CACHE.read().unwrap().get(&key) {
                total += w * 1.12;
                continue;
            }

            // Miss: measure without holding lock
            let glyph = scaled_font.scaled_glyph(c);
            let gb = scaled_font.glyph_bounds(&glyph);
            let measured_w = gb.width();

            // Insert with write-lock; prefer existing if inserted by another thread
            let w = {
                let mut map = UNICODE_WIDTH_CACHE.write().unwrap();
                *map.entry(key).or_insert(measured_w)
            };
            total += w * 1.12;
        }
        total.floor() as usize
    }
}

pub(super) trait SvgPath {
    fn svg_path(&self, height: usize, width: usize) -> String;
}

impl<'a> SvgPath for [f32] {
    fn svg_path(&self, height: usize, width: usize) -> String {
        let len = self.len();
        let chart_height = height as f32;
        let max = self.iter().copied().fold(0.0_f32, f32::max);

        let y_offset = chart_height / max;
        let x_offset = width as f32 / (len as f32 - 1.0);

        // Reserve a larger buffer to reduce reallocations for big series
        // Each segment roughly contributes ~20-30 bytes (command + two floats)
        let mut path_str = String::with_capacity(8 + len * 28);

        for (i, v) in self.iter().enumerate() {
            let x = i as f32 * x_offset;
            let y = chart_height - y_offset * v;
            if i == 0 {
                write!(&mut path_str, "M0 {y}", y = y).unwrap();
            }
            write!(&mut path_str, "L{x} {y}", x = x, y = y).unwrap()
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
