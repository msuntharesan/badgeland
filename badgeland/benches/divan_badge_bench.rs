use badgeland::{Badge, Color, Icon, Size, Style};
use divan::Bencher;
use std::convert::TryFrom;

fn main() {
    divan::main();
}

#[divan::bench]
fn all_text() {
    let mut all_text = Badge::new();
    all_text
        .subject("Hello")
        .color("#6f42c1".parse().unwrap())
        .style(Style::Flat)
        .icon(Icon::try_from("github").unwrap())
        .icon_color("#0366d6".parse().unwrap())
        .size(Size::Large);
    let svg = all_text.text("text content").to_string();
    divan::black_box(svg);
}

#[divan::bench(args = [10, 50, 200, 1000])]
fn all_data(bencher: Bencher, n: i32) {
    // Deterministic data for stable comparisons across runs
    let data: Vec<f32> = (0..n).map(|i| ((i as f32) * 0.123) % 1.0).collect();

    bencher.bench_local(move || {
        let mut all_data = Badge::new();
        all_data
            .subject("Hello")
            .color("#6f42c1".parse().unwrap())
            .style(Style::Flat)
            .icon(Icon::try_from("github").unwrap())
            .icon_color("#0366d6".parse().unwrap())
            .size(Size::Large);
        let d = divan::black_box(&data);
        let svg = all_data.data(d).to_string();
        divan::black_box(svg);
    })
}

#[divan::bench]
fn just_text() {
    let mut just_text = Badge::new();
    just_text
        .color(Color::blue())
        .style(Style::Classic)
        .icon_color(Color::white());
    let svg = just_text.text("Hello").to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn with_text() {
    let mut with_text = Badge::new();
    with_text
        .subject("Hello")
        .color(Color::blue())
        .style(Style::Classic)
        .icon_color(Color::white());
    let svg = with_text.text("text content").to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn medium_size() {
    let mut medium_size = Badge::new();
    medium_size
        .subject("Hello")
        .color(Color::blue())
        .style(Style::Classic)
        .icon_color(Color::white())
        .size(Size::Medium);
    // include wide and emoji characters to exercise unicode sizing
    let svg = medium_size.text("テキスト — text — 中間 🚀").to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn large_size() {
    let mut large_size = Badge::new();
    large_size
        .subject("Hello")
        .color(Color::blue())
        .style(Style::Classic)
        .icon_color(Color::white())
        .size(Size::Large);
    // include several scripts and symbols to test large-size layout
    let svg = large_size
        .text("🚀 Large — 大 — العربية — Русский")
        .to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn red() {
    let mut red = Badge::new();
    red.subject("Hello")
        .color("#ff0000".parse().unwrap())
        .style(Style::Classic)
        .icon_color(Color::white());
    let svg = red.text("red").to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn icon_brand() {
    let mut icon_brand = Badge::new();
    icon_brand
        .subject("Hello")
        .color(Color::blue())
        .style(Style::Classic)
        .icon(Icon::try_from("github").unwrap())
        .icon_color(Color::white());
    let svg = icon_brand.text("brand").to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn icon_solid() {
    let mut icon_solid = Badge::new();
    icon_solid
        .subject("Hello")
        .color(Color::blue())
        .style(Style::Classic)
        .icon(Icon::try_from("code").unwrap())
        .icon_color(Color::white());
    let svg = icon_solid.text("solid").to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn data() {
    let mut data = Badge::new();
    data.subject("Hello")
        .color(Color::blue())
        .style(Style::Classic)
        .icon_color(Color::white());
    let series = [1., 5., 2., 4., 8., 3., 7.];
    let series = divan::black_box(series);
    let svg = data.data(&series).to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn big_data(bencher: Bencher) {
    // Deterministic big data for stable comparisons
    let array: [f32; 30] = core::array::from_fn(|i| ((i as f32) * 0.123) % 1.0);
    bencher.bench_local(move || {
        let mut data = Badge::new();
        data.subject("Hello")
            .color(Color::blue())
            .style(Style::Classic)
            .icon_color(Color::white());
        let arr = divan::black_box(&array);
        let svg = data.data(arr).to_string();
        divan::black_box(svg);
    })
}

#[divan::bench]
fn flat() {
    let mut flat = Badge::new();
    flat.subject("Hello")
        .color(Color::blue())
        .style(Style::Flat)
        .icon_color(Color::white());
    let svg = flat.text("flat").to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn social() {
    let mut social = Badge::new();
    social
        .subject("Hello")
        .color(Color::blue())
        .style(Style::Social)
        .icon_color(Color::white());
    let svg = social.text("social").to_string();
    divan::black_box(svg);
}

// Unicode benches — verify rendering and allocations with non-ASCII text
#[divan::bench]
fn unicode_text() {
    let mut u = Badge::new();
    u.subject("Hello")
        .color(Color::blue())
        .style(Style::Classic)
        .icon_color(Color::white());
    // mixture of scripts and emoji
    let svg = u.text("日本語テキスト 🚀 — привет — 中文").to_string();
    divan::black_box(svg);
}

#[divan::bench]
fn unicode_subject() {
    let mut u = Badge::new();
    u.subject("✓ 日本語 — ✔︎")
        .color(Color::blue())
        .style(Style::Flat)
        .icon_color(Color::white());
    let svg = u.text("Unicode ✓✔︎🚀").to_string();
    divan::black_box(svg);
}
