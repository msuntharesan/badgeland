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

mod badge;
mod badge_data;
mod color;
mod error;
mod icons;

pub use badge::{Badge, Size, Style};
pub use badge_data::BadgeData;
pub use color::*;
pub use error::*;
pub use icons::Icon;

#[cfg(feature = "static_icons")]
pub use icons::{icon_exists, icon_keys};

pub type InitialBadge<'a> = Badge<'a, badge::BadgeTypeInit>;
