use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use merit::{Badge, Color, Icon, Size, Styles, DEFAULT_BLUE, DEFAULT_WHITE};
use std::convert::TryFrom;

pub fn criterion_benchmark(c: &mut Criterion) {
  let all_text = Badge::new()
    .subject("Hello")
    .color(Color("#6f42c1".to_string()))
    .style(Styles::Flat)
    .icon(Icon::try_from("github").unwrap())
    .icon_color(Color("0366d6".to_string()))
    .size(Size::Large)
    .text("text content");

  let all_data = Badge::new()
    .subject("Hello")
    .color(Color("#6f42c1".to_string()))
    .style(Styles::Flat)
    .icon(Icon::try_from("github").unwrap())
    .icon_color(Color("#0366d6".to_string()))
    .size(Size::Large)
    .data(vec![7, 5, 2, 4, 8, 3, 7]);

  let subject = Badge::new()
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Styles::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .text("Hello");

  let with_text = Badge::new()
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Styles::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .text("text content");

  let medium_size = Badge::new()
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Styles::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .size(Size::Medium)
    .text("text content");

  let large_size = Badge::new()
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Styles::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .size(Size::Large)
    .text("text content");

  let red = Badge::new()
    .subject("Hello")
    .color(Color("ff0000".to_string()))
    .style(Styles::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .text("red");

  let icon_brand = Badge::new()
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Styles::Classic)
    .icon(Icon::try_from("github").unwrap())
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .text("brand");

  let icon_solid = Badge::new()
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Styles::Classic)
    .icon(Icon::try_from("code").unwrap())
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .text("solid");

  let data = Badge::new()
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Styles::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .data(vec![1, 5, 2, 4, 8, 3, 7]);

  let flat = Badge::new()
    .subject("Hello")
    .color(DEFAULT_BLUE.parse().unwrap())
    .style(Styles::Classic)
    .icon_color(DEFAULT_WHITE.parse().unwrap())
    .text("flat");

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
