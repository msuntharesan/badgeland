use super::{content::ContentSize, SvgPath};
use crate::badge::BadgeContentType;
use crate::{Color, Icon};
use maud::PreEscaped;
use maud::{html, Markup};

fn content_template<'a>(
    height: usize,
    font_size: f32,
    x_offset: usize,

    icon: Option<(&'a Icon<'a>, &'a Color)>,

    icon_width: usize,

    color: &'a Color,

    content: BadgeContentType<'a>,
    content_size: ContentSize,

    subject: Option<&'a str>,
    subject_size: ContentSize,
) -> Markup {
    html!(
        g#text
            fill=(PreEscaped(Color::white().as_ref()))
            font-family="Verdana,sans-serif"
            font-size=(font_size)
            transform="translate(0, 0)" {
                @if let Some((icon, icon_color)) = &icon {
                use
                    filter="url(#shadow)"
                    xlink:href={"#" (icon.name())}
                    x=(x_offset)
                    y=(((height - icon_width) / 2))
                    width=(icon_width)
                    height=(icon_width)
                    fill=(icon_color.as_ref()) {}
                }
                @if let Some(s) = subject {
                text
                    dominant-baseline="middle"
                    text-anchor="middle"
                    x=(subject_size.x)
                    y=(subject_size.y)
                    filter="url(#shadow)"
                    { (PreEscaped(s)) }
                }
                @if let BadgeContentType::Text(c) = content {
                    text
                        x=((subject_size.rw + content_size.x))
                        y=(content_size.y)
                        text-anchor="middle"
                        dominant-baseline="middle"
                        filter="url(#shadow)"
                        { (PreEscaped(c)) }
                }
                @if let BadgeContentType::Data(d) = content {
                    @let path_str = d.svg_path(height, height * 5);

                    path
                        fill="none"
                        transform=(format!("translate({}, {})", subject_size.rw, 0))
                        stroke=(color.as_ref())
                        stroke-width="1px"
                        d=(PreEscaped(&path_str)) {}
                    path
                        fill=(color.as_ref())
                        fill-opacity="0.2"
                        transform=(PreEscaped(format!("translate({}, {})", subject_size.rw, 0)))
                        stroke="none"
                        stroke-width="0px"
                        d=(PreEscaped(format!("{}V{}H0Z", &path_str, height))) {}
                }
        }
    )
}

pub(super) fn classic_template<'a>(
    width: usize,
    height: usize,
    font_size: f32,
    x_offset: usize,
    rx: usize,

    icon: Option<(&'a Icon<'a>, &'a Color)>,

    icon_width: usize,

    color: &'a Color,

    content: BadgeContentType<'a>,
    content_size: ContentSize,

    subject: Option<&'a str>,
    subject_size: ContentSize,
) -> Markup {
    html!(
        svg
            xmlns:xlink="http://www.w3.org/1999/xlink"
            xmlns="http://www.w3.org/2000/svg"
            viewBox={(format!("0 0 {} {}", width, height))}
            height=(height)
            width=(width) {
                defs {
                    @if let Some((icon, _)) = &icon { (PreEscaped(icon.symbol())) }
                    linearGradient id="a" x2="0" y2="75%"{
                        stop offset="0" stop-color="#eee" stop-opacity="0.1" {}
                        stop offset="1" stop-opacity="0.3" {}
                    }
                    mask id="bg-mask" {
                        rect fill=(PreEscaped(Color::white().as_ref())) height=(height) rx=(rx) width=(width){}
                    }
                    filter id="shadow" {
                        feDropShadow
                            dx="-0.8"
                            dy="-0.8"
                            flood-color=@if content.is_some() { (PreEscaped(Color::black().as_ref())) } @else { (PreEscaped(Color::gray_dark().as_ref())) }
                            flood-opacity="0.4" {}
                    }
                }
                g#bg mask="url(#bg-mask)" {
                    rect fill="url(#a)" height=(height) width=(width) {}
                    @if subject.is_some() || icon.is_some() {
                        rect#subject
                            fill=@if content.is_some() { (PreEscaped(Color::gray_dark().as_ref())) } @else { (color.as_ref()) }
                            height=(height)
                            width=(subject_size.rw) {}
                    }
                    rect#content
                        fill=@match &content{
                            BadgeContentType::Data(_) => { (PreEscaped(Color::gray().as_ref())) }
                            _ => (color.as_ref())
                        }
                        height=(height)
                        width=(content_size.rw)
                        x=(subject_size.rw) {}
                }
                (content_template(
                    height,
                    font_size,
                    x_offset,

                    icon,

                    icon_width,

                    color,

                    content,
                    content_size,

                    subject,
                    subject_size,
                ))
        }
    )
}

pub(super) fn flat_template<'a>(
    width: usize,
    height: usize,
    font_size: f32,
    x_offset: usize,

    icon: Option<(&'a Icon<'a>, &'a Color)>,

    icon_width: usize,

    color: &'a Color,

    content: BadgeContentType<'a>,
    content_size: ContentSize,

    subject: Option<&'a str>,
    subject_size: ContentSize,
) -> Markup {
    html!(
        svg
            xmlns:xlink="http://www.w3.org/1999/xlink"
            xmlns="http://www.w3.org/2000/svg"
            viewBox={(format!("0 0 {} {}", width, height))}
            height=(height)
            width=(width) {
                defs {
                    @if let Some((icon, _)) = &icon { (PreEscaped(icon.symbol())) }
                    filter id="shadow" {
                        feDropShadow
                            dx="-0.8"
                            dy="-0.8"
                            stdDeviation="0"
                            flood-color=@if content.is_some() { (PreEscaped(Color::black().as_ref())) } @else { (PreEscaped(Color::gray_dark().as_ref())) }
                            flood-opacity="0.4" {}
                    }
                }
                g#bg {
                    rect fill=(PreEscaped(Color::gray().as_ref())) height=(height) width=(width) {}
                    @if subject.is_some() || icon.is_some() {
                        rect#subject
                            fill=@if content.is_some() { (PreEscaped(Color::gray_dark().as_ref())) } @else { (color.as_ref()) }
                            height=(height)
                            width=(subject_size.rw) {}
                    }
                    rect#content
                        fill=@match &content {
                            BadgeContentType::Data(_) => { (PreEscaped(Color::gray().as_ref())) }
                            _ => (color.as_ref())
                        }
                        height=(height)
                        width=(content_size.rw)
                        x=(subject_size.rw) {}
                }
                (content_template(
                    height,
                    font_size,
                    x_offset,

                    icon,

                    icon_width,

                    color,

                    content,
                    content_size,

                    subject,
                    subject_size,
                ))
        }
    )
}
