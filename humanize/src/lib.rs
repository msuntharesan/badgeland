/*!
Formatter for human readable number or struct

This crate exposes `humanize` fn to format numbers to human readable string

# Quick Start

Add `humanize` to your `Cargo.toml` as as a dependency.

# Examples

```rust
use humanize::*;

fn main() {
  let opts = HumanizeOptions::builder().build();
  let human_readable = 1234;
  assert_eq!(human_readable.humanize(opts), "1.23K".to_string())
}
```

*/

use std::usize;

use once_cell::sync::Lazy;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone)]
pub enum Base {
    Base2,
    Base10,
}

impl Default for Base {
    fn default() -> Self {
        Base::Base10
    }
}

impl From<usize> for Base {
    fn from(b: usize) -> Self {
        match b {
            2 => Self::Base2,
            _ => Self::Base10,
        }
    }
}

impl Base {
    #[inline]
    fn get_denominator(&self) -> usize {
        match self {
            Base::Base2 => 2_usize.pow(10),
            Base::Base10 => 10_usize.pow(3),
        }
    }
}

/// Options to pass to humanize function
#[derive(Debug, TypedBuilder)]
pub struct HumanizeOptions {
    #[builder(setter(into), default = Base::Base10)]
    base: Base,
    #[builder(default = 2)]
    precision: usize,
    #[builder(default = false)]
    keep_zero: bool,
    #[builder(default = ".")]
    decimal_separator: &'static str,
    #[builder(default = false)]
    lower_case: bool,
    #[builder(default = false)]
    space: bool,
    #[builder(default = vec!["", "K", "M", "B", "T", "q", "Q"])]
    units: Vec<&'static str>,
}

impl HumanizeOptions {
    #[inline]
    fn denominator(&self) -> usize {
        self.base.get_denominator()
    }
}

pub static DEFAULT_OPTIONS: Lazy<HumanizeOptions> = Lazy::new(|| HumanizeOptions::builder().build());

pub static DEFAULT_BYTES_OPTIONS: Lazy<HumanizeOptions> = Lazy::new(|| {
    HumanizeOptions::builder()
        .units(vec!["B", "KB", "MB", "GB", "TB", "PB", "EB"])
        .build()
});

pub static DEFAULT_BITS_OPTIONS: Lazy<HumanizeOptions> = Lazy::new(|| {
    HumanizeOptions::builder()
        .base(2)
        .units(vec!["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"])
        .build()
});

impl AsRef<HumanizeOptions> for HumanizeOptions {
    fn as_ref(&self) -> &HumanizeOptions {
        self
    }
}

/// Trait that can be implemented for any type.
pub trait Humanize {
    /// formats a type to human readable string
    fn humanize<T: AsRef<HumanizeOptions>>(&self, opts: T) -> String;
}

macro_rules! impl_humanize_u {
    (for $($t: ty)*) => ($(
        impl Humanize for $t {
            #[inline]
            fn humanize<T: AsRef<HumanizeOptions>>(&self, opts: T) -> String {
                let opts = opts.as_ref();
                let denominator = opts.denominator() as f64;

                let mut val = *self as f64;
                let mut unit = 0;
                while val >= denominator {
                    val /= denominator;
                    unit += 1;
                }
                let mut suffix = if unit > opts.units.len() {
                    opts.units.last().unwrap().to_string()
                } else {
                    opts.units[unit].to_owned()
                };

                if opts.lower_case {
                    suffix = suffix.to_lowercase();
                }

                let fract = (val.fract() * 10_f64.powi(opts.precision as i32)).round() / 10_f64.powi(opts.precision as i32);

                let precision: usize = if fract == 0.0 && !opts.keep_zero { 0 } else { opts.precision as usize };

                let space = if opts.space { " " } else { "" };
                let mut formatted:String = format!("{:.*}{}{}", precision , val, space, suffix);
                if opts.decimal_separator != "." {
                    formatted = formatted.replace(".", opts.decimal_separator);
                }
                formatted
            }
        }
    )*)
}

macro_rules! impl_humanize_i {
    (for $($t: ty)*) => ($(
        impl Humanize for $t {
            #[inline]
            fn humanize<T: AsRef<HumanizeOptions>>(&self, _opts: T) -> String{
                let opts: &HumanizeOptions = _opts.as_ref();
                let sign = if *self < 0 { "-" } else { "" };
                format!("{}{}", sign, (self.abs() as u64).humanize(opts))
            }
        }
    )*)
}

macro_rules! impl_humanize_f {
    (for $($t: ty)*) => ($(
        impl Humanize for $t {
            #[inline]
            fn humanize<T: AsRef<HumanizeOptions>>(&self, _opts: T) -> String{
                let opts: &HumanizeOptions = _opts.as_ref();
                let sign = if *self < 0.0 { "-" } else { "" };
                format!("{}{}", sign, (self.abs() as u64).humanize(opts))
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
    fn test_denominator() {
        let opt = HumanizeOptions::builder()
            .base(2)
            .units(vec!["B", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei"])
            .build();
        assert_eq!(100.humanize(&opt), "100B".to_owned());
        assert_eq!(1_024.humanize(&opt), "1Ki".to_owned());
        assert_eq!(1_050_000.humanize(&opt), "1Mi".to_owned());
        assert_eq!(1_080_000_000.humanize(&opt), "1.01Gi".to_owned());
        assert_eq!(1_100_000_000_000u64.humanize(&opt), "1Ti".to_owned());
    }

    #[test]
    fn test_usize() {
        let opt = HumanizeOptions::builder().build();
        assert_eq!(100.humanize(&opt), "100".to_owned());
        assert_eq!(1_000.humanize(&opt), "1K".to_owned());
        assert_eq!(1_000_000.humanize(&opt), "1M".to_owned());
        assert_eq!(1_000_000_000.humanize(&opt), "1B".to_owned());
        assert_eq!(1_000_000_000_000u64.humanize(&opt), "1T".to_owned());
    }

    #[test]
    fn test_isize() {
        let opt = HumanizeOptions::builder().build();
        assert_eq!((-100).humanize(&opt), "-100".to_owned());
        assert_eq!((100).humanize(&opt), "100".to_owned());
        assert_eq!((-1000).humanize(&opt), "-1K".to_owned());
        assert_eq!((-1000000).humanize(&opt), "-1M".to_owned());
        assert_eq!((-1000000000).humanize(&opt), "-1B".to_owned());
        assert_eq!((-1000000000000i64).humanize(&opt), "-1T".to_owned());
    }

    #[test]
    fn test_floats() {
        let opt = HumanizeOptions::builder().build();
        assert_eq!((-100f32).humanize(&opt), "-100".to_owned());
        assert_eq!((100f32).humanize(&opt), "100".to_owned());
        assert_eq!((-1000f32).humanize(&opt), "-1K".to_owned());
        assert_eq!((-1000000f32).humanize(&opt), "-1M".to_owned());
        assert_eq!((-1000000000f32).humanize(&opt), "-1B".to_owned());
        assert_eq!((-1000000000000f64).humanize(&opt), "-1T".to_owned());
        assert_eq!((-12345.678f32).humanize(&opt), "-12.35K".to_owned())
    }

    #[test]
    fn test_lowercase_suffix() {
        let opt = HumanizeOptions::builder().lower_case(true).build();

        assert_eq!(1000.humanize(&opt), "1k".to_owned());
        assert_eq!(1000000.humanize(&opt), "1m".to_owned());
        assert_eq!(1000000000.humanize(&opt), "1b".to_owned());
        assert_eq!(1000000000000u64.humanize(&opt), "1t".to_owned());
    }

    #[test]
    fn test_precision() {
        let value = 12345.6789;

        assert_eq!(
            value.humanize(&HumanizeOptions::builder().precision(0usize).build()),
            "12K".to_owned()
        );
        assert_eq!(
            value.humanize(&HumanizeOptions::builder().precision(1usize).build()),
            "12.3K".to_owned()
        );
        assert_eq!(
            value.humanize(&HumanizeOptions::builder().precision(2usize).build()),
            "12.35K".to_owned()
        );
        assert_eq!(
            value.humanize(&HumanizeOptions::builder().precision(3usize).build()),
            "12.345K".to_owned()
        );
    }

    #[test]
    fn test_precision_with_zero() {
        let opt = HumanizeOptions::builder().precision(1usize).build();

        assert_eq!(1010000000.humanize(&opt), "1B".to_owned());
        assert_eq!(1060000000.humanize(&opt), "1.1B".to_owned());
        assert_eq!(1810000000.humanize(&opt), "1.8B".to_owned());

        let opt = HumanizeOptions::builder().keep_zero(true).precision(1usize).build();

        assert_eq!(1010000000.humanize(&opt), "1.0B".to_owned());
        assert_eq!(1060000000.humanize(&opt), "1.1B".to_owned());
        assert_eq!(1810000000.humanize(&opt), "1.8B".to_owned());

        let opt = HumanizeOptions::builder().keep_zero(false).precision(2usize).build();

        assert_eq!(1001000000.humanize(&opt), "1B".to_owned());
        assert_eq!(1060000000.humanize(&opt), "1.06B".to_owned());
        assert_eq!(1810000000.humanize(&opt), "1.81B".to_owned());

        let opt = HumanizeOptions::builder().precision(2usize).keep_zero(true).build();

        assert_eq!(1001000000.humanize(&opt), "1.00B".to_owned());
        assert_eq!(1060000000.humanize(&opt), "1.06B".to_owned());
        assert_eq!(1810000000.humanize(&opt), "1.81B".to_owned());

        let opt = HumanizeOptions::builder().keep_zero(false).precision(3usize).build();

        assert_eq!(1000100000.humanize(&opt), "1B".to_owned());
        assert_eq!(1060000000.humanize(&opt), "1.060B".to_owned());
        assert_eq!(1813450000.humanize(&opt), "1.813B".to_owned());

        let opt = HumanizeOptions::builder().keep_zero(true).precision(3usize).build();

        assert_eq!(1000100000.humanize(&opt), "1.000B".to_owned());
        assert_eq!(1060000000.humanize(&opt), "1.060B".to_owned());
        assert_eq!(1813450000.humanize(&opt), "1.813B".to_owned());
    }

    #[test]
    fn test_decimal_separator() {
        let value = 12345.6789;
        let opt = HumanizeOptions::builder().decimal_separator("_").build();
        assert_eq!(value.humanize(&opt), "12_35K".to_owned());
    }

    #[test]
    fn test_units() {
        let value = 123450.6789;
        let opt = HumanizeOptions::builder().units(vec!["m", "km"]).build();
        assert_eq!(value.humanize(&opt), "123.45km".to_owned());
    }
}
