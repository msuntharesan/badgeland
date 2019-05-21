#[derive(Debug)]
  pub struct HumanizeOptions {
    base: usize,
    precision: usize,
    decimal_separator: &'static str,
    // decimal_zeroes: usize,
    lower_case: bool,
    space: bool,
    units: Vec<&'static str>,
  }

  impl Default for HumanizeOptions {
    fn default() -> Self {
      HumanizeOptions {
        base: 1000,
        precision: 2,
        decimal_separator: ".",
        // decimal_zeroes: 0,
        lower_case: false,
        space: false,
        units: vec!["", "K", "M", "B", "T", "P", "E"],
      }
    }
  }


  impl AsRef<HumanizeOptions> for HumanizeOptions {
    fn as_ref(&self) -> &HumanizeOptions {
      self
    }
  }


  pub fn humanize_options() -> HumanizeOptions {
    HumanizeOptions::default()
  }

  impl HumanizeOptions {
    pub fn new() -> Self {
      HumanizeOptions::default()
    }
    pub fn set_base(&mut self, base: usize) -> &mut Self {
      self.base = base;
      self
    }
    pub fn set_precision(&mut self, precision: usize) -> &mut Self {
      self.precision = precision;
      self
    }
    pub fn set_decimal_separator(&mut self, decimal_separator: &'static str) -> &mut Self {
      self.decimal_separator = decimal_separator;
      self
    }
    pub fn set_lowercase(&mut self, lower_case: bool) -> &mut Self {
      self.lower_case = lower_case;
      self
    }
    pub fn set_space(&mut self, space: bool) -> &mut Self {
      self.space = space;
      self
    }
    pub fn set_units(&mut self, units: Vec<&'static str>) -> &mut Self {
      self.units = units;
      self
    }
  }

  pub trait Humanize {
    fn humanize<T: AsRef<HumanizeOptions>>(&self, opts: T) -> Option<String>;
  }


  macro_rules! impl_humanize_u {
  (for $($t: ty)*) => ($(
    impl Humanize for $t {
      fn humanize<T: AsRef<HumanizeOptions>>(&self, _opts: T) -> Option<String>{
        let opts: &HumanizeOptions = _opts.as_ref();
        let denominator = opts.base as f64;

        let mut val: f64 = *self as f64;
        let mut unit = 0;
        while val>= opts.base as f64{
          val /= denominator;
          unit += 1;
        }
        unit = if unit > opts.units.len() { opts.units.len() - 1 } else { unit };
        let mut suffix: String = opts.units[unit].to_owned();
        if opts.lower_case {
          suffix = suffix.to_lowercase();
        }

        let precision = if val.fract() == 0.0 { 0 } else { opts.precision };
        let space = if opts.space { " " } else { "" };
        let mut formatted:String = format!("{:.*}{}{}", precision, val, space, suffix);
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
      fn humanize <T: AsRef<HumanizeOptions>>(&self, _opts: T) -> Option<String>{
        let opts = _opts.as_ref();
        let sign = if *self < 0 { "-" } else { "" };
        Some(format!("{}{}", sign, (self.abs() as u64).humanize(opts).unwrap()))
      }
    }
  )*)
}

  macro_rules! impl_humanize_f {
  (for $($t: ty)*) => ($(
    impl Humanize for $t {
      fn humanize <T: AsRef<HumanizeOptions>>(&self, _opts: T) -> Option<String>{
        let opts = _opts.as_ref();
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
    let opt = HumanizeOptions::default();
    assert_eq!(100.humanize(&opt), Some("100".to_owned()));
    assert_eq!(1000.humanize(&opt), Some("1K".to_owned()));
    assert_eq!(1000000.humanize(&opt), Some("1M".to_owned()));
    assert_eq!(1000000000.humanize(&opt), Some("1B".to_owned()));
    assert_eq!(1000000000000u64.humanize(&opt), Some("1T".to_owned()));
    assert_eq!(12345.678.humanize(&opt), Some("12.35K".to_owned()))
  }
  #[test]
  fn test_isize() {
    let opt = HumanizeOptions::default();
    assert_eq!((-100).humanize(&opt), Some("-100".to_string()));
    assert_eq!((-100).humanize(&opt), Some("-100".to_owned()));
    assert_eq!((-1000).humanize(&opt), Some("-1K".to_owned()));
    assert_eq!((-1000000).humanize(&opt), Some("-1M".to_owned()));
    assert_eq!((-1000000000).humanize(&opt), Some("-1B".to_owned()));
    assert_eq!((-1000000000000i64).humanize(&opt), Some("-1T".to_owned()));
    assert_eq!((-12345.678).humanize(&opt), Some("-12.35K".to_owned()))
  }
  #[test]
  fn test_lowercase_suffix() {
    let mut opt = HumanizeOptions::default(); //.set_lower_case(true).get();
    opt.set_lowercase(true);

    assert_eq!(1000.humanize(&opt), Some("1k".to_owned()));
    assert_eq!(1000000.humanize(&opt), Some("1m".to_owned()));
    assert_eq!(1000000000.humanize(&opt), Some("1b".to_owned()));
    assert_eq!(1000000000000u64.humanize(&opt), Some("1t".to_owned()));
  }
  #[test]
  fn test_precision() {
    let value = 12345.6789;
    let mut opt = HumanizeOptions::new(); //.set_lower_case(true).get();
    opt.set_precision(0);
    assert_eq!(value.humanize(&opt), Some("12K".to_owned()));
    opt.set_precision(1);
    assert_eq!(value.humanize(&opt), Some("12.3K".to_owned()));
    opt.set_precision(2);
    assert_eq!(value.humanize(&opt), Some("12.35K".to_owned()));
    opt.set_precision(3);
    assert_eq!(value.humanize(&opt), Some("12.345K".to_owned()));
  }

  #[test]
  fn test_decimal_separator() {
    let value = 12345.6789;
    let mut opt = HumanizeOptions::new(); //.set_lower_case(true).get();
    opt.set_decimal_separator("_");
    assert_eq!(value.humanize(&opt), Some("12_35K".to_owned()));
  }
  #[test]
  fn test_units() {
    let value = 123450.6789;
    let mut opt = HumanizeOptions::new(); //.set_lower_case(true).get();
    opt.set_units(vec!["m", "km"]);
    assert_eq!(value.humanize(&opt), Some("123.45km".to_owned()));
  }
}