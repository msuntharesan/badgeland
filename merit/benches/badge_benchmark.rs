use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use merit::{Badge, Color, Icon, Size, Style, DEFAULT_BLUE, DEFAULT_WHITE};
use std::convert::TryFrom;

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut all_text = Badge::new();

  all_text
    .subject("Hello")
    .color(Color("#6f42c1".to_string()))
    .style(Style::Flat)
    .icon(Icon::try_from("github").unwrap())
    .icon_color(Color("0366d6".to_string()))
    .size(Size::Large);

  let all_text = all_text.text("text content");

  let mut all_data = Badge::new();
  all_data
    .subject("Hello")
    .color(Color("#6f42c1".to_string()))
    .style(Style::Flat)
    .icon(Icon::try_from("github").unwrap())
    .icon_color(Color("#0366d6".to_string()))
    .size(Size::Large);
  let all_data = all_data.data(&[7., 5., 2., 4., 8., 3., 7.]);

  let mut subject = Badge::new();
  subject
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Style::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap());
  let subject = subject.text("Hello");

  let mut with_text = Badge::new();
  with_text
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Style::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap());
  let with_text = with_text.text("text content");

  let mut medium_size = Badge::new();
  medium_size
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Style::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .size(Size::Medium);
  let medium_size = medium_size.text("text content");

  let mut large_size = Badge::new();
  large_size
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Style::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .size(Size::Large);
  let large_size = large_size.text("text content");

  let mut red = Badge::new();
  red
    .subject("Hello")
    .color(Color("ff0000".to_string()))
    .style(Style::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap());
  let red = red.text("red");

  let mut icon_brand = Badge::new();
  icon_brand
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Style::Classic)
    .icon(Icon::try_from("github").unwrap())
    .icon_color(DEFAULT_WHITE.parse().unwrap());
  let icon_brand = icon_brand.text("brand");

  let mut icon_solid = Badge::new();
  icon_solid
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Style::Classic)
    .icon(Icon::try_from("code").unwrap())
    .icon_color(DEFAULT_WHITE.parse().unwrap());
  let icon_solid = icon_solid.text("solid");

  let mut data = Badge::new();
  data
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Style::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap());
  let data = data.data(&[1., 5., 2., 4., 8., 3., 7.]);

  let mut flat = Badge::new();
  flat
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Style::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap());
  let flat = flat.text("flat");

  c.bench_with_input(BenchmarkId::new("badge", "all_text"), &all_text, |b, badge| {
    b.iter(|| badge.to_string())
  });

  c.bench_with_input(BenchmarkId::new("badge", "all_data"), &all_data, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "subject"), &subject, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "with_text"), &with_text, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "medium_size"), &medium_size, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "large_size"), &large_size, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "red"), &red, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "icon_brand"), &icon_brand, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "icon_solid"), &icon_solid, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "data"), &data, |b, badge| {
    b.iter(|| badge.to_string())
  });
  c.bench_with_input(BenchmarkId::new("badge", "flat"), &flat, |b, badge| {
    b.iter(|| badge.to_string())
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
