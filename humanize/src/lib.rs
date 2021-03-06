//! Formatter for human readable number or struct
//!
//! This crate exposes `humanize` fn to format numbers to human readable string
//!
//! # Quick Start
//!
//! Add `humanize` to your `Cargo.toml` as as a dependency.
//!
//! # Examples
//!
//! ```rust
//! use humanize::*;
//!
//! fn main() {
//!   let opts = HumanizeOptions::builder().build().unwrap();
//!   let human_readable = 1234;
//!   assert_eq!(human_readable.humanize(opts), Some("1.23K".to_string()))
//! }
//! ```
//!

use derive_builder::Builder;

/// Options to pass to humanize function
#[derive(Debug, Builder)]
pub struct HumanizeOptions {
    #[builder(default = "1000")]
    denominator: usize,
    #[builder(default = "2")]
    precision: usize,
    #[builder(default = "false")]
    keep_zero: bool,
    #[builder(default = r#"".""#)]
    decimal_separator: &'static str,
    #[builder(default = "false")]
    lower_case: bool,
    #[builder(default = "false")]
    space: bool,
    #[builder(default = r#"vec!["", "K", "M", "B", "T", "P", "E"]"#)]
    units: Vec<&'static str>,
}

impl HumanizeOptions {
    /// Create a builder for HumanizeOptions
    pub fn builder() -> HumanizeOptionsBuilder {
        HumanizeOptionsBuilder::default()
    }
}

impl AsRef<HumanizeOptions> for HumanizeOptions {
    fn as_ref(&self) -> &HumanizeOptions {
        self
    }
}

/// Trait that can be implemented for any type.
pub trait Humanize {
    /// formats a type to human readable string
    fn humanize<T: AsRef<HumanizeOptions>>(&self, opts: T) -> Option<String>;
}

macro_rules! impl_humanize_u {
  (for $($t: ty)*) => ($(
    impl Humanize for $t {
      fn humanize<T: AsRef<HumanizeOptions>>(&self, opts: T) -> Option<String>{
        let opts = opts.as_ref();
        let denominator = opts.denominator as f64;

        let mut val: f64 = *self as f64;
        let mut unit = 0;
        while val>= denominator as f64 {
          val /= denominator;
          unit += 1;
        }
        let mut suffix:String = if unit > opts.units.len() {
          opts.units.last().unwrap().to_string()
        } else {
          opts.units[unit].to_owned()
        };

        if opts.lower_case {
          suffix = suffix.to_lowercase();
        }

        let fract = (val.fract() * (10.0f64).powi(opts.precision as i32)).round() / 10.0f64.powi(opts.precision as i32);

        let precision: usize = if fract == 0.0 && !opts.keep_zero { 0 } else { opts.precision as usize };

        let space = if opts.space { " " } else { "" };
        let mut formatted:String = format!("{:.*}{}{}", precision , val, space, suffix);
        if opts.decimal_separator != "." {
          formatted = formatted.replace(".", opts.decimal_separator);
        }
        Some(formatted)
      }
    }
  )*)
}

macro_rules! impl_humanize_i {
  (for $($t: ty)*) => ($(
    impl Humanize for $t {
      fn humanize<T: AsRef<HumanizeOptions>>(&self, _opts: T) -> Option<String>{
        let opts: &HumanizeOptions = _opts.as_ref();
        let sign = if *self < 0 { "-" } else { "" };
        Some(format!("{}{}", sign, (self.abs() as u64).humanize(opts).unwrap()))
      }
    }
  )*)
}

macro_rules! impl_humanize_f {
  (for $($t: ty)*) => ($(
    impl Humanize for $t {
      fn humanize<T: AsRef<HumanizeOptions>>(&self, _opts: T) -> Option<String>{
        let opts: &HumanizeOptions = _opts.as_ref();
        let sign = if *self < 0.0 { "-" } else { "" };
        Some(format!("{}{}", sign, (self.abs() as u64).humanize(opts).unwrap()))
      }
    }
  )*)
}

impl_humanize_u!(for usize u8 u16 u32 u64);
impl_humanize_i!(for isize i8 i16 i32 i64);
impl_humanize_f!(for f32 f64);

#[cfg(test)]
mod tests {
    use super::{Humanize, HumanizeOptions};

    #[test]
    fn test_usize() {
        let opt = HumanizeOptions::builder().build().unwrap();
        assert_eq!(100.humanize(&opt), Some("100".to_owned()));
        assert_eq!(1000.humanize(&opt), Some("1K".to_owned()));
        assert_eq!(1000000.humanize(&opt), Some("1M".to_owned()));
        assert_eq!(1000000000.humanize(&opt), Some("1B".to_owned()));
        assert_eq!(1000000000000u64.humanize(&opt), Some("1T".to_owned()));
    }

    #[test]
    fn test_isize() {
        let opt = HumanizeOptions::builder().build().unwrap();
        assert_eq!((-100).humanize(&opt), Some("-100".to_string()));
        assert_eq!((100).humanize(&opt), Some("100".to_owned()));
        assert_eq!((-1000).humanize(&opt), Some("-1K".to_owned()));
        assert_eq!((-1000000).humanize(&opt), Some("-1M".to_owned()));
        assert_eq!((-1000000000).humanize(&opt), Some("-1B".to_owned()));
        assert_eq!((-1000000000000i64).humanize(&opt), Some("-1T".to_owned()));
    }

    #[test]
    fn test_floats() {
        let opt = HumanizeOptions::builder().build().unwrap();
        assert_eq!((-100f32).humanize(&opt), Some("-100".to_string()));
        assert_eq!((100f32).humanize(&opt), Some("100".to_owned()));
        assert_eq!((-1000f32).humanize(&opt), Some("-1K".to_owned()));
        assert_eq!((-1000000f32).humanize(&opt), Some("-1M".to_owned()));
        assert_eq!((-1000000000f32).humanize(&opt), Some("-1B".to_owned()));
        assert_eq!((-1000000000000f64).humanize(&opt), Some("-1T".to_owned()));
        assert_eq!((-12345.678f32).humanize(&opt), Some("-12.35K".to_owned()))
    }

    #[test]
    fn test_lowercase_suffix() {
        let opt = HumanizeOptions::builder().lower_case(true).build().unwrap();

        assert_eq!(1000.humanize(&opt), Some("1k".to_owned()));
        assert_eq!(1000000.humanize(&opt), Some("1m".to_owned()));
        assert_eq!(1000000000.humanize(&opt), Some("1b".to_owned()));
        assert_eq!(1000000000000u64.humanize(&opt), Some("1t".to_owned()));
    }

    #[test]
    fn test_precision() {
        let value = 12345.6789;

        let mut opts = HumanizeOptions::builder();
        assert_eq!(
            value.humanize(&opts.precision(0usize).build().unwrap()),
            Some("12K".to_owned())
        );
        assert_eq!(
            value.humanize(&opts.precision(1usize).build().unwrap()),
            Some("12.3K".to_owned())
        );
        assert_eq!(
            value.humanize(&opts.precision(2usize).build().unwrap()),
            Some("12.35K".to_owned())
        );
        assert_eq!(
            value.humanize(&opts.precision(3usize).build().unwrap()),
            Some("12.345K".to_owned())
        );
    }

    #[test]
    fn test_precision_with_zero() {
        let mut opt_builder = HumanizeOptions::builder();
        let opt = opt_builder.precision(1usize).build().unwrap();

        assert_eq!(1010000000.humanize(&opt), Some("1B".to_owned()));
        assert_eq!(1060000000.humanize(&opt), Some("1.1B".to_owned()));
        assert_eq!(1810000000.humanize(&opt), Some("1.8B".to_owned()));

        let opt = opt_builder.keep_zero(true).precision(1usize).build().unwrap();

        assert_eq!(1010000000.humanize(&opt), Some("1.0B".to_owned()));
        assert_eq!(1060000000.humanize(&opt), Some("1.1B".to_owned()));
        assert_eq!(1810000000.humanize(&opt), Some("1.8B".to_owned()));

        let opt = opt_builder.keep_zero(false).precision(2usize).build().unwrap();

        assert_eq!(1001000000.humanize(&opt), Some("1B".to_owned()));
        assert_eq!(1060000000.humanize(&opt), Some("1.06B".to_owned()));
        assert_eq!(1810000000.humanize(&opt), Some("1.81B".to_owned()));

        let opt = opt_builder.precision(2usize).keep_zero(true).build().unwrap();

        assert_eq!(1001000000.humanize(&opt), Some("1.00B".to_owned()));
        assert_eq!(1060000000.humanize(&opt), Some("1.06B".to_owned()));
        assert_eq!(1810000000.humanize(&opt), Some("1.81B".to_owned()));

        let opt = opt_builder.keep_zero(false).precision(3usize).build().unwrap();

        assert_eq!(1000100000.humanize(&opt), Some("1B".to_owned()));
        assert_eq!(1060000000.humanize(&opt), Some("1.060B".to_owned()));
        assert_eq!(1813450000.humanize(&opt), Some("1.813B".to_owned()));

        let opt = opt_builder.keep_zero(true).precision(3usize).build().unwrap();

        assert_eq!(1000100000.humanize(&opt), Some("1.000B".to_owned()));
        assert_eq!(1060000000.humanize(&opt), Some("1.060B".to_owned()));
        assert_eq!(1813450000.humanize(&opt), Some("1.813B".to_owned()));
    }

    #[test]
    fn test_decimal_separator() {
        let value = 12345.6789;
        let opt = HumanizeOptions::builder().decimal_separator("_").build().unwrap();
        assert_eq!(value.humanize(&opt), Some("12_35K".to_owned()));
    }

    #[test]
    fn test_units() {
        let value = 123450.6789;
        let opt = HumanizeOptions::builder().units(vec!["m", "km"]).build().unwrap();
        assert_eq!(value.humanize(&opt), Some("123.45km".to_owned()));
    }
}
