mod content;
mod size;
mod style;
use sailfish::TemplateOnce;

pub use size::Size;

pub use style::Style;

use super::{icons::Icon, Color};
use content::{BadgeContentSize, ContentSize, SvgPath, TextWidth};
use core::{f32, fmt};
use std::fmt::Debug;

#[derive(Debug)]
pub struct BadgeTypeInit;
#[derive(Debug)]
pub struct BadgeTypeData<'a>(&'a [f32]);
#[derive(Debug)]
pub struct BadgeTypeText<'a>(&'a str);

pub trait BadgeType<'a> {
    fn content(&self) -> BadgeContentType<'_>;
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
            BadgeContentType::Text(c) => {
                c.content_size(height, c.text_width(font_size), padding, 0)
            }
            _ => ContentSize::default(),
        }
    }
}

impl BadgeType<'_> for BadgeTypeInit {
    #[inline]
    fn content(&self) -> BadgeContentType<'_> {
        BadgeContentType::None
    }
}

impl<'a> BadgeType<'a> for BadgeTypeData<'a> {
    #[inline]
    fn content(&self) -> BadgeContentType<'a> {
        BadgeContentType::Data(self.0)
    }
}

impl<'a> BadgeType<'a> for BadgeTypeText<'a> {
    #[inline]
    fn content(&self) -> BadgeContentType<'a> {
        BadgeContentType::Text(self.0)
    }
}

#[derive(Debug)]
pub struct Badge<'a, S: BadgeType<'a> = BadgeTypeInit> {
    subject: Option<&'a str>,
    color: Color,
    style: Style,
    icon: Option<Icon<'a>>,
    icon_color: Color,
    size: Size,
    content: S,
}

impl<'a> Badge<'a> {
    pub fn new() -> Self {
        Badge {
            subject: None,
            color: Color::blue(),
            style: Style::Classic,
            icon: None,
            icon_color: Color::white(),
            size: Size::Small,
            content: BadgeTypeInit,
        }
    }
}

impl<'a> Badge<'a, BadgeTypeInit> {
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
    fn height(&self) -> usize {
        match self.size {
            Size::Small => 20,
            Size::Medium => 30,
            Size::Large => 40,
        }
    }

    #[inline]
    fn font_size(&self) -> f32 {
        self.height() as f32 * SVG_FONT_MULTIPLIER
    }

    #[inline]
    fn icon_size(&self) -> (usize, usize) {
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
        let height = self.height();

        let font_size = self.font_size();

        let (icon_width, x_offset) = self.icon_size();

        match self.subject {
            Some(s) => s.content_size(
                height,
                s.text_width(font_size),
                padding,
                x_offset + icon_width,
            ),
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

#[derive(TemplateOnce)]
#[template(path = "classic.stpl")]
struct ClassicTemplate<'a> {
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
}

#[derive(TemplateOnce)]
#[template(path = "flat.stpl")]
struct FlatTemplate<'a> {
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
}

#[derive(TemplateOnce)]
#[template(path = "social.stpl")]
struct SocialTemplate<'a> {
    width: usize,
    height: usize,
    font_size: f32,
    x_offset: usize,
    rx: usize,

    icon: Option<(&'a Icon<'a>, Option<&'a Color>)>,

    icon_width: usize,

    color: &'a Color,

    content: BadgeContentType<'a>,
    content_size: ContentSize,

    subject: Option<&'a str>,
    subject_size: ContentSize,
}

const SVG_FONT_MULTIPLIER: f32 = 0.65;

impl<'a, T: BadgeType<'a>> Badge<'a, T> {
    #[inline]
    fn render(&self) -> String {
        let height = self.height();

        let font_size = self.font_size();

        let padding = height / 2;

        let (icon_width, x_offset) = self.icon_size();

        let subject_size = self.subject_size(padding);

        let content = self.content.content();

        let content_size = content.content_size(height, padding, font_size);

        let mut width = subject_size.rw + content_size.rw;

        // Social style reserves extra space for the split notch and borders
        if matches!(self.style, Style::Social) {
            width += 7; // approximates 6px notch + 1px stroke
        }

        let rx = self.rx();

        let icon = self.icon.as_ref().map(|i| (i, &self.icon_color));

        match self.style {
            Style::Classic => {
                let tpl = ClassicTemplate {
                    width,
                    height,
                    font_size,
                    x_offset,
                    rx,
                    icon,
                    icon_width,
                    color: &self.color,
                    content,
                    content_size,
                    subject: self.subject,
                    subject_size,
                };
                tpl.render_once().unwrap()
            }
            Style::Flat => {
                let tpl = FlatTemplate {
                    width,
                    height,
                    font_size,
                    x_offset,
                    icon,
                    icon_width,
                    color: &self.color,
                    content,
                    content_size,
                    subject: self.subject,
                    subject_size,
                };
                tpl.render_once().unwrap()
            }
            Style::Social => {
                let social_icon = self.icon.as_ref().map(|i| {
                    if self.icon_color == Color::white() {
                        (i, None)
                    } else {
                        (i, Some(&self.icon_color))
                    }
                });
                let tpl = SocialTemplate {
                    width,
                    height,
                    font_size,
                    x_offset,
                    rx,
                    icon: social_icon,
                    icon_width,
                    color: &self.color,
                    content,
                    content_size,
                    subject: self.subject,
                    subject_size,
                };
                tpl.render_once().unwrap()
            }
        }
    }
}

impl<'a, T: BadgeType<'a>> fmt::Display for Badge<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::{style::Style, Badge, Color, Size};
    use crate::Icon;
    use scraper::{Html, Selector};
    use std::convert::TryFrom;

    #[test]
    fn default_badge_has_classic_style() {
        let mut badge = Badge::new();
        badge.subject("just text");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        assert_eq!(badge.style, Style::Classic, "style not Classic");
        let lg_selector = Selector::parse("linearGradient").unwrap();
        assert!(doc.select(&lg_selector).next().is_some());
    }
    #[test]
    fn default_badge_has_20px_height() {
        let mut badge = Badge::new();
        badge.subject("just text");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        let selector = Selector::parse("svg").unwrap();
        let svg = doc.select(&selector).next().unwrap();
        assert_eq!(svg.value().attr("height"), Some("20"));
    }
    #[test]
    fn default_badge_only_has_subject() {
        let mut badge = Badge::new();
        badge.subject("just subject");
        let badge_svg = badge.to_string();
        let doc = Html::parse_fragment(&badge_svg);
        let text_sel = Selector::parse("g#text > text").unwrap();
        let text_els = doc.select(&text_sel);
        assert_eq!(text_els.count(), 1);
        let text = doc.select(&text_sel).next().unwrap();
        assert_eq!(
            text.text().collect::<String>().trim(),
            String::from("just subject")
        );
    }
    #[test]
    fn default_badge_has_333_as_background_color() {
        let mut badge = Badge::new();
        badge.subject("just text");
        badge.color(Color::blue());
        let def_color: Color = Color::blue();
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
        assert_eq!(
            subject.text().collect::<String>().trim(),
            String::from("badge text")
        );
    }

    #[test]
    fn classic_has_a11y_labels() {
        let mut badge = Badge::new();
        badge.subject("Any text");
        let svg = badge.text("you like").to_string();
        let doc = Html::parse_fragment(&svg);
        let svg_sel = Selector::parse("svg").unwrap();
        let svg_el = doc.select(&svg_sel).next().unwrap();
        assert_eq!(svg_el.value().attr("role"), Some("img"));
        assert_eq!(svg_el.value().attr("aria-label"), Some("Any text: you like"));
        let title_sel = Selector::parse("title").unwrap();
        let title = doc.select(&title_sel).next().unwrap();
        assert_eq!(title.text().collect::<String>(), "Any text: you like");
    }

    #[test]
    fn flat_has_a11y_labels() {
        let mut badge = Badge::new();
        badge
            .subject("Any text")
            .style(Style::Flat);
        let svg = badge.text("you like").to_string();
        let doc = Html::parse_fragment(&svg);
        let svg_sel = Selector::parse("svg").unwrap();
        let svg_el = doc.select(&svg_sel).next().unwrap();
        assert_eq!(svg_el.value().attr("role"), Some("img"));
        assert_eq!(svg_el.value().attr("aria-label"), Some("Any text: you like"));
        let title_sel = Selector::parse("title").unwrap();
        let title = doc.select(&title_sel).next().unwrap();
        assert_eq!(title.text().collect::<String>(), "Any text: you like");
    }

    #[test]
    fn social_has_a11y_and_notch_elements() {
        let mut badge = Badge::new();
        badge
            .subject("Any text")
            .style(Style::Social);
        let svg = badge.text("you like").to_string();
        let doc = Html::parse_fragment(&svg);
        let svg_sel = Selector::parse("svg").unwrap();
        let svg_el = doc.select(&svg_sel).next().unwrap();
        assert_eq!(svg_el.value().attr("role"), Some("img"));
        assert_eq!(svg_el.value().attr("aria-label"), Some("Any text: you like"));
        // Notch/gloss indicator elements present
        let gloss_sel = Selector::parse("#llink").unwrap();
        assert!(doc.select(&gloss_sel).next().is_some());
        let style_sel = Selector::parse("style").unwrap();
        let style_el = doc.select(&style_sel).next().unwrap();
        let style_txt = style_el.text().collect::<String>();
        assert!(style_txt.contains("#llink"));
    }

    #[test]
    #[cfg(feature = "static_icons")]
    fn badge_with_icon() {
        let mut badge = Badge::new();
        badge
            .subject("with icon")
            .icon(Icon::try_from("git").unwrap());

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
        let mut badge = Badge::new();
        badge.icon(Icon::try_from("git").unwrap());

        let icon = &badge.icon;
        assert!(icon.is_some());

        let doc = Html::parse_fragment(&badge.to_string());
        let icon_sel = Selector::parse("symbol").unwrap();
        let icon_symbol = doc.select(&icon_sel).next().unwrap();
        assert_eq!(icon_symbol.value().attr("id"), Some("git"));
    }

    #[test]
    #[cfg(feature = "static_icons")]
    fn social_default_icon_color_is_gray_dark() {
        let mut badge = Badge::new();
        badge
            .subject("icon")
            .style(Style::Social)
            .icon(Icon::try_from("git").unwrap());
        // Do not set icon_color explicitly
        let svg = badge.text("text").to_string();
        let doc = Html::parse_fragment(&svg);
        let use_sel = Selector::parse("use").unwrap();
        let u = doc.select(&use_sel).next().unwrap();
        assert_eq!(u.value().attr("fill"), Some(Color::gray_dark().as_ref()));
    }

    #[test]
    #[cfg(feature = "static_icons")]
    fn social_respects_explicit_icon_color() {
        let mut badge = Badge::new();
        badge
            .subject("icon")
            .style(Style::Social)
            .icon(Icon::try_from("git").unwrap())
            .icon_color(Color::black());
        let svg = badge.text("text").to_string();
        let doc = Html::parse_fragment(&svg);
        let use_sel = Selector::parse("use").unwrap();
        let u = doc.select(&use_sel).next().unwrap();
        assert_eq!(u.value().attr("fill"), Some(Color::black().as_ref()));
    }

    #[test]
    fn badge_has_medium_icon() {
        let mut badge = Badge::new();
        badge.subject("with icon").size(Size::Medium);
        let doc = Html::parse_fragment(&badge.to_string());
        let svg_sel = Selector::parse("svg").unwrap();
        let svg = doc.select(&svg_sel).next().unwrap();
        assert_eq!(svg.value().attr("height"), Some("30"));
    }
    #[test]
    fn badge_has_large_icon() {
        let mut badge = Badge::new();
        badge.subject("with icon").size(Size::Large);
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
