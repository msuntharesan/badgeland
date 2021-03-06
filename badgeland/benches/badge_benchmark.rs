use badgeland::{Badge, Color, Icon, Size, Style, DEFAULT_BLUE, DEFAULT_WHITE};
use criterion::{criterion_group, criterion_main, Criterion};
use std::convert::TryFrom;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("badges");

    group
        .bench_function("all_text", |b| {
            b.iter(|| {
                let mut all_text = Badge::new();
                all_text
                    .subject("Hello")
                    .color(Color("#6f42c1".to_string()))
                    .style(Style::Flat)
                    .icon(Icon::try_from("github").unwrap())
                    .icon_color(Color("0366d6".to_string()))
                    .size(Size::Large);
                all_text.text("text content").to_string();
            })
        })
        .bench_function("all_data", |b| {
            b.iter(|| {
                let mut all_data = Badge::new();
                all_data
                    .subject("Hello")
                    .color(Color("#6f42c1".to_string()))
                    .style(Style::Flat)
                    .icon(Icon::try_from("github").unwrap())
                    .icon_color(Color("#0366d6".to_string()))
                    .size(Size::Large);
                all_data.data(&[7., 5., 2., 4., 8., 3., 7.]).to_string();
            })
        })
        .bench_function("just_text", |b| {
            b.iter(|| {
                let mut just_text = Badge::new();
                just_text
                    .color(DEFAULT_BLUE.parse().unwrap())
                    .style(Style::Classic)
                    .icon_color(DEFAULT_WHITE.parse().unwrap());
                just_text.text("Hello").to_string();
            })
        })
        .bench_function("with_text", |b| {
            b.iter(|| {
                let mut with_text = Badge::new();
                with_text
                    .subject("Hello")
                    .color(DEFAULT_BLUE.parse().unwrap())
                    .style(Style::Classic)
                    .icon_color(DEFAULT_WHITE.parse().unwrap());
                with_text.text("text content").to_string();
            })
        })
        .bench_function("medium_size", |b| {
            b.iter(|| {
                let mut medium_size = Badge::new();
                medium_size
                    .subject("Hello")
                    .color(DEFAULT_BLUE.parse().unwrap())
                    .style(Style::Classic)
                    .icon_color(DEFAULT_WHITE.parse().unwrap())
                    .size(Size::Medium);
                medium_size.text("text content").to_string();
            })
        })
        .bench_function("large_size", |b| {
            b.iter(|| {
                let mut large_size = Badge::new();
                large_size
                    .subject("Hello")
                    .color(DEFAULT_BLUE.parse().unwrap())
                    .style(Style::Classic)
                    .icon_color(DEFAULT_WHITE.parse().unwrap())
                    .size(Size::Large);
                large_size.text("text content").to_string();
            })
        })
        .bench_function("red", |b| {
            b.iter(|| {
                let mut red = Badge::new();
                red.subject("Hello")
                    .color(Color("ff0000".to_string()))
                    .style(Style::Classic)
                    .icon_color(DEFAULT_WHITE.parse().unwrap());
                red.text("red").to_string();
            })
        })
        .bench_function("icon_brand", |b| {
            b.iter(|| {
                let mut icon_brand = Badge::new();
                icon_brand
                    .subject("Hello")
                    .color(DEFAULT_BLUE.parse().unwrap())
                    .style(Style::Classic)
                    .icon(Icon::try_from("github").unwrap())
                    .icon_color(DEFAULT_WHITE.parse().unwrap());
                icon_brand.text("brand").to_string();
            })
        })
        .bench_function("icon_solid", |b| {
            b.iter(|| {
                let mut icon_solid = Badge::new();
                icon_solid
                    .subject("Hello")
                    .color(DEFAULT_BLUE.parse().unwrap())
                    .style(Style::Classic)
                    .icon(Icon::try_from("code").unwrap())
                    .icon_color(DEFAULT_WHITE.parse().unwrap());
                icon_solid.text("solid").to_string();
            })
        })
        .bench_function("data", |b| {
            b.iter(|| {
                let mut data = Badge::new();
                data.subject("Hello")
                    .color(DEFAULT_BLUE.parse().unwrap())
                    .style(Style::Classic)
                    .icon_color(DEFAULT_WHITE.parse().unwrap());
                data.data(&[1., 5., 2., 4., 8., 3., 7.]).to_string();
            })
        })
        .bench_function("flat", |b| {
            b.iter(|| {
                let mut flat = Badge::new();
                flat.subject("Hello")
                    .color(DEFAULT_BLUE.parse().unwrap())
                    .style(Style::Classic)
                    .icon_color(DEFAULT_WHITE.parse().unwrap());
                flat.text("flat").to_string();
            })
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
