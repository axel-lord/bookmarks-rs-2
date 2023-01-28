//! Some utilities to make working with ui easier.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use iced::{
    widget::{Column, Row},
    Color, Element,
};

pub mod tabs;
pub mod text_button;

/// Extension trait to create rows or columns from an iterator.
pub trait IteratorWidgetExt<Message>: Iterator {
    /// Collect an iterator into a column using the passed function to transform the iterator
    /// content to elements.
    fn collect_column<'a, E, F>(self, f: F) -> Column<'a, Message>
    where
        E: Into<Element<'a, Message>>,
        F: FnMut(Self::Item) -> E;

    /// Collect an iterator into a row using the passed function to transform the iterator content
    /// to elements
    fn collect_row<'a, E, F>(self, f: F) -> Row<'a, Message>
    where
        E: Into<Element<'a, Message>>,
        F: FnMut(Self::Item) -> E;
}

impl<I, Message> IteratorWidgetExt<Message> for I
where
    I: Iterator,
{
    fn collect_row<'a, E, F>(self, mut f: F) -> Row<'a, Message>
    where
        E: Into<Element<'a, Message>>,
        F: FnMut(Self::Item) -> E,
    {
        self.fold(Row::new(), |row, item| row.push(f(item)))
    }

    fn collect_column<'a, E, F>(self, mut f: F) -> Column<'a, Message>
    where
        E: Into<Element<'a, Message>>,
        F: FnMut(Self::Item) -> E,
    {
        self.fold(Column::new(), |column, item| column.push(f(item)))
    }
}

/// Renderer to use theme.
pub type Renderer = iced::Renderer<Theme>;

/// Custom theme used for ui.
#[derive(Clone, Copy, Debug, Default)]
pub enum Theme {
    /// Light theme
    #[default]
    Light,
    /// Dark theme
    Dark,
    /// Muted dark theme
    DarkMute,
}

/// Configuration for boxes.
#[derive(Clone, Copy, Debug)]
pub struct BoxOptions {
    /// Whether to show a border, only applies if possible.
    pub border: bool,
    /// Whether to set text color, only applies if possible.
    pub text: bool,
    /// Whether to set background color, only applies if possible.
    pub background: bool,
}

impl BoxOptions {
    /// Keep text color, no border, no backgorund.
    #[must_use]
    pub fn minimal() -> Self {
        Self {
            border: false,
            text: false,
            background: false,
        }
    }
}

impl Default for BoxOptions {
    fn default() -> Self {
        Self::minimal()
    }
}

/// A simple box palette constisting of colors for background text and border.
#[derive(Clone, Copy, Debug)]
pub struct ContrastPalette {
    /// Bright color of this palette, in light mode is background.
    pub bright: Color,
    /// Dim color of this palette, in light mode is text and border.
    pub dim: Color,
}

impl ContrastPalette {
    /// Get a monochrome (black and white) color palette.
    #[must_use]
    pub fn monochrome() -> Self {
        Self {
            bright: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            dim: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct BoxPalette {
    text: Color,
    background: Color,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let [a, b, t] = [a, b, t].map(|n| n.clamp(0.0, 1.0));
    (a + t * (b - a)).clamp(0.0, 1.0)
}

fn mute_color(Color { r, g, b, a }: Color) -> Color {
    Color {
        r: lerp(r, 0.5, 0.25),
        g: lerp(g, 0.5, 0.25),
        b: lerp(b, 0.5, 0.25),
        a,
    }
}

impl BoxPalette {
    fn from_contrast_palette(theme: Theme, palette: ContrastPalette) -> Self {
        match theme {
            Theme::Light => BoxPalette {
                text: palette.dim,
                background: palette.bright,
            },
            Theme::Dark => BoxPalette {
                text: palette.bright,
                background: palette.dim,
            },
            Theme::DarkMute => BoxPalette {
                text: palette.bright,
                background: mute_color(palette.bright),
            },
        }
    }

    fn invert(self) -> Self {
        let Self { text, background } = self;
        Self {
            text: background,
            background: text,
        }
    }
}

impl Default for ContrastPalette {
    fn default() -> Self {
        Self::monochrome()
    }
}

pub mod theme {
    //! different themes for widgets.
}

impl iced::application::StyleSheet for Theme {
    type Style = ContrastPalette;

    fn appearance(&self, style: &Self::Style) -> iced::application::Appearance {
        let palette = BoxPalette::from_contrast_palette(*self, *style);
        iced::application::Appearance {
            background_color: palette.background,
            text_color: palette.text,
        }
    }
}

impl iced::widget::text::StyleSheet for Theme {
    type Style = (ContrastPalette, BoxOptions);

    fn appearance(&self, style: Self::Style) -> iced::widget::text::Appearance {
        let palette = BoxPalette::from_contrast_palette(*self, style.0);
        iced::widget::text::Appearance {
            color: style.1.text.then_some(palette.text),
        }
    }
}

impl iced::widget::container::StyleSheet for Theme {
    type Style = (ContrastPalette, BoxOptions);

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        let palette = BoxPalette::from_contrast_palette(*self, style.0);
        iced::widget::container::Appearance {
            border_radius: 0.0,
            border_width: if style.1.border { 1.0 } else { 0.0 },
            text_color: style.1.text.then_some(palette.text),
            background: style.1.background.then_some(palette.background.into()),
            border_color: palette.text,
        }
    }
}

impl iced::widget::toggler::StyleSheet for Theme {
    type Style = (ContrastPalette, BoxOptions);

    fn active(&self, style: &Self::Style, _is_active: bool) -> iced::widget::toggler::Appearance {
        let palette = BoxPalette::from_contrast_palette(*self, style.0).invert();
        iced::widget::toggler::Appearance {
            background: palette.background,
            foreground: palette.text,
            background_border: None,
            foreground_border: None,
        }
    }

    fn hovered(&self, style: &Self::Style, _is_active: bool) -> iced::widget::toggler::Appearance {
        let palette = BoxPalette::from_contrast_palette(
            *self,
            ContrastPalette {
                bright: style.0.bright,
                dim: mute_color(style.0.dim),
            },
        )
        .invert();
        iced::widget::toggler::Appearance {
            background: palette.background,
            foreground: palette.text,
            background_border: None,
            foreground_border: None,
        }
    }
}
