mod content;
mod size;
mod style;

pub use size::Size;

pub use style::Style;
use style::{classic_template, flat_template};

use super::{icons::Icon, Color, DEFAULT_BLACK, DEFAULT_BLUE, DEFAULT_GRAY, DEFAULT_GRAY_DARK, DEFAULT_WHITE};
use content::{BadgeContentSize, ContentSize, Path, TextWidth};
use core::f32;
use std::fmt;

#[derive(Debug)]
pub struct BadgeTypeInit;
#[derive(Debug)]
pub struct BadgeTypeData<'a>(&'a [f32]);
#[derive(Debug)]
pub struct BadgeTypeText<'a>(&'a str);

pub trait BadgeType<'a> {
    fn content(&self) -> BadgeContentType;
}

#[derive(Debug)]
pub enum BadgeContentType<'a> {
    None,
    Text(&'a str),
    Data(&'a [f32]),
}

impl BadgeContentType<'_> {
    #[inline]
    fn is_some(&self) -> bool {
        !matches!(self, BadgeContentType::None)
    }

    #[inline]
    fn content_size(&self, height: usize, padding: usize, font_size: f32) -> ContentSize {
        match self {
            BadgeContentType::Data(d) => d.content_size(height, height * 5, padding, 0),
            BadgeContentType::Text(c) => c.content_size(height, c.get_text_width(font_size), padding, 0),
            _ => ContentSize::default(),
        }
    }

    #[inline]
    fn path_str(self, height: usize, width: usize) -> Option<String> {
        match self {
            BadgeContentType::Data(d) => {
                let mut path_str = String::new();
                let mut path_iter = Path::new(d, height, width);
                let (x, y) = path_iter.next().unwrap();
                path_str.push_str(&format!("M0 {y}L{x} {y}", x = x, y = y));

                for (x, y) in path_iter {
                    path_str.push_str(&format!("L{x} {y}", x = x, y = y));
                }
                Some(path_str)
            }
            _ => None,
        }
    }
}

impl BadgeType<'_> for BadgeTypeInit {
    #[inline]
    fn content(&self) -> BadgeContentType {
        BadgeContentType::None
    }
}

impl<'a> BadgeType<'a> for BadgeTypeData<'a> {
    #[inline]
    fn content(&self) -> BadgeContentType {
        BadgeContentType::Data(self.0)
    }
}

impl<'a> BadgeType<'a> for BadgeTypeText<'a> {
    #[inline]
    fn content(&self) -> BadgeContentType {
        BadgeContentType::Text(self.0)
    }
}

#[derive(Debug)]
pub struct Badge<'a, S: BadgeType<'a>> {
    subject: Option<&'a str>,
    color: Color,
    style: Style,
    icon: Option<Icon<'a>>,
    icon_color: Color,
    size: Size,
    content: S,
}

impl<'a> Badge<'a, BadgeTypeInit> {
    pub fn new() -> Self {
        Badge {
            subject: None,
            color: DEFAULT_BLUE.parse().unwrap(),
            style: Style::Classic,
            icon: None,
            icon_color: DEFAULT_WHITE.parse().unwrap(),
            size: Size::Small,
            content: BadgeTypeInit,
        }
    }

    pub fn subject(&mut self, subject: &'a str) -> &mut Self {
        self.subject = Some(subject);
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn icon(&mut self, icon: Icon<'a>) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = size;
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn icon_color(&mut self, c: Color) -> &mut Self {
        if let Some(_) = &self.icon {
            self.icon_color = c;
        }
        self
    }

    pub fn text(self, text: &'a str) -> Badge<'a, BadgeTypeText<'a>> {
        Badge {
            subject: self.subject,
            color: self.color,
            style: self.style,
            icon: self.icon,
            icon_color: self.icon_color,
            size: self.size,
            content: BadgeTypeText(text),
        }
    }

    pub fn data(self, data: &'a [f32]) -> Badge<'a, BadgeTypeData<'a>> {
        Badge {
            subject: self.subject,
            color: self.color,
            style: self.style,
            icon: self.icon,
            icon_color: self.icon_color,
            size: self.size,
            content: BadgeTypeData(data),
        }
    }
}

impl<'a, T: BadgeType<'a>> Badge<'a, T> {
    #[inline]
    fn get_height(&self) -> usize {
        match self.size {
            Size::Small => 20,
            Size::Medium => 30,
            Size::Large => 40,
        }
    }

    #[inline]
    fn get_font_size(&self) -> f32 {
        self.get_height() as f32 * SVG_FONT_MULTIPLIER
    }

    #[inline]
    fn get_icon_size(&self) -> (usize, usize) {
        if self.icon.is_none() {
            return (0, 0);
        }
        match self.size {
            Size::Large => (30, 10),
            Size::Medium => (20, 8),
            Size::Small => (15, 5),
        }
    }

    #[inline]
    fn subject_size(&self, padding: usize) -> ContentSize {
        let height = self.get_height();

        let font_size = self.get_font_size();

        let (icon_width, x_offset) = self.get_icon_size();

        match self.subject {
            Some(s) => s.content_size(height, s.get_text_width(font_size), padding, x_offset + icon_width),
            None if self.icon.is_some() => ContentSize {
                rw: icon_width + x_offset * 2,
                x: x_offset,
                y: height,
            },
            _ => ContentSize::default(),
        }
    }

    #[inline]
    fn rx(&self) -> usize {
        match self.size {
            Size::Medium => 6,
            Size::Large => 9,
            _ => 3,
        }
    }
}

const SVG_FONT_MULTIPLIER: f32 = 0.65;

impl<'a, T: BadgeType<'a>> fmt::Display for Badge<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self.get_height();

        let font_size = self.get_font_size();

        let padding = height / 2;

        let (icon_width, x_offset) = self.get_icon_size();

        let subject_size = self.subject_size(padding);

        let content = self.content.content();

        let content_size = content.content_size(height, padding, font_size);

        let width = subject_size.rw + content_size.rw;

        let rx = self.rx();

        let markup = match self.style {
            Style::Classic => classic_template(
                width,
                height,
                font_size,
                x_offset,
                rx,
                self.icon.as_ref().map(|i| (i, &self.icon_color)),
                icon_width,
                &self.color,
                content,
                content_size,
                self.subject,
                subject_size,
            ),
            Style::Flat => flat_template(
                width,
                height,
                font_size,
                x_offset,
                self.icon.as_ref().map(|i| (i, &self.icon_color)),
                icon_width,
                &self.color,
                content,
                content_size,
                self.subject,
                subject_size,
            ),
        };

        write!(f, "{}", markup.into_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{style::Style, Badge, Color, Size, DEFAULT_BLUE};
    use scraper::{Html, Selector};

    use crate::Icon;
    use std::convert::TryFrom;

    #[test]
    fn default_badge_has_classic_style() {
        let mut badge = Badge::new();
        &badge.subject("just text");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        assert_eq!(badge.style, Style::Classic, "style not Classic");
        let lg_selector = Selector::parse("linearGradient").unwrap();
        assert!(doc.select(&lg_selector).next().is_some());
    }
    #[test]
    fn default_badge_has_20px_height() {
        let mut badge = Badge::new();
        &badge.subject("just text");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        let selector = Selector::parse("svg").unwrap();
        let svg = doc.select(&selector).next().unwrap();
        assert_eq!(svg.value().attr("height"), Some("20"));
    }
    #[test]
    fn default_badge_only_has_subject() {
        let mut badge = Badge::new();
        &badge.subject("just subject");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        let text_sel = Selector::parse("g#text > text").unwrap();
        let text_els = doc.select(&text_sel);
        assert_eq!(text_els.count(), 1);
        let text = doc.select(&text_sel).next().unwrap();
        assert_eq!(text.text().collect::<String>(), String::from("just subject"));
    }
    #[test]
    fn default_badge_has_333_as_background_color() {
        let mut badge = Badge::new();
        &badge.subject("just text");
        badge.color(DEFAULT_BLUE.parse().unwrap());
        let def_color: Color = DEFAULT_BLUE.parse().unwrap();
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        let rect_sel = Selector::parse("g#bg > rect#subject").unwrap();
        let rect = doc.select(&rect_sel).next().unwrap();
        assert_eq!(rect.value().attr("fill").unwrap(), def_color.as_ref());
    }

    #[test]
    fn badge_with_text() {
        let mut badge = Badge::new();
        badge.subject("with subject");
        let doc = Html::parse_fragment(&badge.text("badge text").to_string());
        let subject_sel = Selector::parse("g#text > text:last-child").unwrap();
        let subject = doc.select(&subject_sel).next().unwrap();
        assert_eq!(subject.text().collect::<String>(), String::from("badge text"));
    }

    #[test]
    #[cfg(feature = "static_icons")]
    fn badge_with_icon() {
        let icon = Icon::try_from("git").unwrap();
        let mut badge = Badge::new();
        &badge.subject("with icon").icon(icon);

        let icon = &badge.icon;
        assert!(icon.is_some());

        let doc = Html::parse_fragment(&badge.to_string());
        let icon_sel = Selector::parse("symbol").unwrap();
        let icon_symbol = doc.select(&icon_sel).next().unwrap();
        assert_eq!(icon_symbol.value().attr("id"), Some("git"));
    }

    #[test]
    #[cfg(feature = "static_icons")]
    fn badge_with_icon_only() {
        let icon = Icon::try_from("git").unwrap();
        let mut badge = Badge::new();
        &badge.icon(icon);

        let icon = &badge.icon;
        assert!(icon.is_some());

        let doc = Html::parse_fragment(&badge.to_string());
        let icon_sel = Selector::parse("symbol").unwrap();
        let icon_symbol = doc.select(&icon_sel).next().unwrap();
        assert_eq!(icon_symbol.value().attr("id"), Some("git"));
    }

    #[test]
    fn badge_has_medium_icon() {
        let mut badge = Badge::new();
        &badge.subject("with icon").size(Size::Medium);
        let doc = Html::parse_fragment(&badge.to_string());
        let svg_sel = Selector::parse("svg").unwrap();
        let svg = doc.select(&svg_sel).next().unwrap();
        assert_eq!(svg.value().attr("height"), Some("30"));
    }
    #[test]
    fn badge_has_large_icon() {
        let mut badge = Badge::new();
        &badge.subject("with icon").size(Size::Large);
        let doc = Html::parse_fragment(&badge.to_string());
        let svg_sel = Selector::parse("svg").unwrap();
        let svg = doc.select(&svg_sel).next().unwrap();
        assert_eq!(svg.value().attr("height"), Some("40"));
    }

    #[test]
    fn badge_with_data() {
        let mut badge = Badge::new();
        badge.subject("Some data");
        let badge = badge.data(&[1., 2., 3., 4., 5.]);

        let doc = Html::parse_fragment(&badge.to_string());
        println!("{:?}", &badge.to_string());
        let line_sel = Selector::parse("path").unwrap();
        let svg = doc.select(&line_sel).next().unwrap();
        assert!(svg.value().attr("d").is_some());
    }
}
