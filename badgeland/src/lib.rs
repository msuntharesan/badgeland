/*!
Fast badge generator for any purpose

Create badges with text, icons and sparkline chart

# Web

See <https://github.com/msuntharesan/badgeland#web>

# Quick start

Add `badgeland` to your `Cargo.toml` as as a dependency.

# Examples

```rust
use badgeland::{Badge};

fn badge() {
  let mut badge = Badge::new();
  badge.subject("Subject");
  println!("{}", badge.text("Text").to_string());
}
```
This produce a svg badge: ![](https://badge.land/b/Subject/Text)
```rust
use badgeland::{Badge};

fn badge_with_data() {
  let mut badge = Badge::new();
  badge.subject("Subject");
  println!("{}", badge.data(&[12., 34., 23., 56., 45.]).to_string());
}
```
This produce a svg badge: ![](http://badge.land/b/testing/12,34,23,56,45)

*/

use std::{convert::From, num::ParseFloatError, str::FromStr};

#[cfg(feature = "serde_de")]
use serde::Deserialize;

mod badge;
mod color;
mod icons;

pub use badge::{Badge, Size, Style};

pub use color::*;

#[cfg(feature = "static_icons")]
pub use icons::{icon_exists, icon_keys};

pub use icons::Icon;

pub type InitialBadge<'a> = Badge<'a, badge::BadgeTypeInit>;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde_de", derive(Deserialize))]
pub struct BadgeData(pub Vec<f32>);

impl FromStr for BadgeData {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(",")
            .map(|s| s.trim().parse::<f32>())
            .collect::<Result<Vec<_>, Self::Err>>()
            .map(|values| BadgeData(values))
    }
}

impl From<Vec<f32>> for BadgeData {
    fn from(values: Vec<f32>) -> Self {
        BadgeData(values)
    }
}

#[cfg(test)]
mod tests {

    use super::BadgeData;

    #[test]
    fn data_from_string_fails() {
        let d = "not a number".parse::<BadgeData>();
        assert!(d.is_err());
        let d = "12,12,,12".parse::<BadgeData>();
        assert!(d.is_err());
    }

    #[test]
    fn data_from_string_parse_correct() {
        let d = "12,23, 23, 12".parse::<BadgeData>();
        assert!(d.is_ok());
        assert_eq!(d.unwrap().0, vec![12., 23., 23., 12.]);
    }

    #[test]
    fn dat_from_json_parse_fails() {
        let d = "12, 32!,23, 23, 12".parse::<BadgeData>();
        assert!(d.is_err());
    }
}
