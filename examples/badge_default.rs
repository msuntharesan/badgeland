extern crate badge_maker;

use badge_maker::Badge;

fn main() {
  let badge = Badge::new("Badge Maker");

  println!("{}", badge);
}
