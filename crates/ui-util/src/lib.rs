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
use tap::Pipe;

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

impl Theme {
    /// Get [`BoxPalette`] representing current theme.
    #[must_use]
    pub fn box_palette(&self) -> BoxPalette {
        let ContrastPalette { bright, dim } = self.contrast_palette();
        match self {
            Theme::Light => BoxPalette {
                text: dim,
                background: bright,
            },
            Theme::Dark => BoxPalette {
                text: bright,
                background: dim,
            },
            Theme::DarkMute => BoxPalette {
                text: bright,
                background: mute_color(dim, None),
            },
        }
    }

    /// Get [`ContrastPalette`] representing current theme.
    #[must_use]
    pub fn contrast_palette(&self) -> ContrastPalette {
        ContrastPalette::monochrome()
    }
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

    /// Do not keep text color, no border, use backgorund.
    #[must_use]
    pub fn solid() -> Self {
        Self {
            border: false,
            text: true,
            background: true,
        }
    }

    /// Do not keep text color, border, use backgorund.
    #[must_use]
    pub fn defined() -> Self {
        Self {
            border: true,
            text: true,
            background: true,
        }
    }

    /// Construct with border set to passed value and the other fields default constructed.
    #[must_use]
    pub fn with_border(border: bool) -> Self {
        Self {
            border,
            ..Default::default()
        }
    }

    /// Construct with text set to passed value and the other fields default constructed.
    #[must_use]
    pub fn with_text(text: bool) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    /// Construct with background set to passed value and the other fields default constructed.
    #[must_use]
    pub fn with_background(background: bool) -> Self {
        Self {
            background,
            ..Default::default()
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

    /// Mute the dim color.
    #[must_use]
    pub fn mute_dim(self, t: Option<f32>) -> Self {
        Self {
            dim: mute_color(self.dim, t),
            ..self
        }
    }

    /// Swap the dim and bright fields.
    #[must_use]
    pub fn invert(self) -> Self {
        let Self { bright, dim } = self;
        Self {
            bright: dim,
            dim: bright,
        }
    }
}

/// A Palette with values for foreground/text and background.
#[derive(Debug, Clone, Copy)]
pub struct BoxPalette {
    /// Foreground/text/outline color.
    pub text: Color,
    /// Background color.
    pub background: Color,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let [a, b, t] = [a, b, t].map(|n| n.clamp(0.0, 1.0));
    (a + t * (b - a)).clamp(0.0, 1.0)
}

fn mute_color(Color { r, g, b, a }: Color, t: Option<f32>) -> Color {
    let t = t.unwrap_or(0.25);
    Color {
        r: lerp(r, 0.5, t),
        g: lerp(g, 0.5, t),
        b: lerp(b, 0.5, t),
        a,
    }
}

impl BoxPalette {
    /// Create a [`BoxPalette`] from a [`ContrastPalette`] and a theme.
    #[must_use]
    pub fn from_contrast_palette(theme: Theme, palette: ContrastPalette) -> Self {
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
                background: mute_color(palette.dim, None),
            },
        }
    }

    /// Swap background and foreground.
    #[must_use]
    pub fn invert(self) -> Self {
        let Self { text, background } = self;
        Self {
            text: background,
            background: text,
        }
    }

    /// Mute the background color somewhat.
    #[must_use]
    pub fn mute_background(self, t: Option<f32>) -> Self {
        let Self { text, background } = self;
        Self {
            text,
            background: mute_color(background, t),
        }
    }
}

impl Default for ContrastPalette {
    fn default() -> Self {
        Self::monochrome()
    }
}

pub mod theme {
    //! Different themes for widgets.

    use crate::{BoxOptions, BoxPalette, ContrastPalette, Theme};
    use bookmark_util::Somewhere;
    use iced::Color;
    // use paste::paste;

    /// Style used for [Container][iced::widget::Container] widgets
    #[derive(Default)]
    pub enum Container {
        /// Use the default style of the current theme.
        #[default]
        Theme,
        /// Use a palette based on contrast swapping what is foreground and background based on
        /// theme
        ContrastPalette(ContrastPalette, BoxOptions),
        /// Use a palette where the foreground and background values are set.
        BoxPalette(BoxPalette, BoxOptions),
        /// Implement the style yourself and pass it.
        Custom(Somewhere<dyn iced::widget::container::StyleSheet<Style = Theme>>),
    }

    /// Style used for [Text][iced::widget::Text] widgets.
    #[derive(Default, Clone, Copy, Debug)]
    pub enum Text {
        /// Use the default style of the current theme.
        #[default]
        Theme,
        /// Use a palette based on contrast swapping what is foreground and background based on
        /// theme
        ContrastPalette(ContrastPalette),
        /// Set the text color to the passed color.
        Color(Color),
    }
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
    type Style = Option<theme::Text>;

    fn appearance(&self, style: Self::Style) -> iced::widget::text::Appearance {
        use iced::widget::text::Appearance;
        Appearance {
            color: style.map(|style| match style {
                theme::Text::Theme => todo!(),
                theme::Text::ContrastPalette(palette) => match self {
                    Theme::Light => palette.dim,
                    Theme::Dark => palette.bright,
                    Theme::DarkMute => mute_color(palette.dim, None),
                },
                theme::Text::Color(color) => color,
            }),
        }
    }
}

impl iced::widget::container::StyleSheet for Theme {
    type Style = theme::Container;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        // let palette = BoxPalette::from_contrast_palette(*self, style.palette);
        // iced::widget::container::Appearance {
        //     border_radius: 0.0,
        //     border_width: if style.options.border { 1.0 } else { 0.0 },
        //     text_color: style.options.text.then_some(palette.text),
        //     background: style
        //         .options
        //         .background
        //         .then_some(palette.background.into()),
        //     border_color: palette.text,
        // }
        todo!()
    }
}

impl iced::widget::toggler::StyleSheet for Theme {
    type Style = theme::Container;

    fn active(&self, style: &Self::Style, _is_active: bool) -> iced::widget::toggler::Appearance {
        // let palette = BoxPalette::from_contrast_palette(*self, style.palette).pipe(|p| {
        //     if style.options.border {
        //         p
        //     } else {
        //         p.invert()
        //     }
        // });
        // iced::widget::toggler::Appearance {
        //     background: palette.background,
        //     foreground: palette.text,
        //     background_border: style.options.border.then_some(palette.text),
        //     foreground_border: style.options.border.then_some(palette.background),
        // }
        todo!()
    }

    fn hovered(&self, style: &Self::Style, _is_active: bool) -> iced::widget::toggler::Appearance {
        // let palette = BoxPalette::from_contrast_palette(*self, style.palette.mute_dim(None))
        //     .pipe(|p| if style.options.border { p } else { p.invert() });
        // iced::widget::toggler::Appearance {
        //     background: palette.background,
        //     foreground: palette.text,
        //     background_border: style.options.border.then_some(palette.text),
        //     foreground_border: style.options.border.then_some(palette.background),
        // }
        todo!()
    }
}

impl iced::widget::button::StyleSheet for Theme {
    type Style = theme::Container;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        // let palette = BoxPalette::from_contrast_palette(*self, style.palette).pipe(|p| {
        //     if style.options.border || (!style.options.background) {
        //         p
        //     } else {
        //         p.invert()
        //     }
        // });
        // iced::widget::button::Appearance {
        //     background: style
        //         .options
        //         .background
        //         .then_some(palette.background.into()),
        //     border_radius: 3.0,
        //     border_width: if style.options.border { 1.0 } else { 0.0 },
        //     border_color: palette.text,
        //     text_color: palette.text,
        //     ..Default::default()
        // }
        todo!()
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        // let palette = BoxPalette::from_contrast_palette(*self, style.palette)
        //     .pipe(|p| {
        //         if style.options.border || (!style.options.background) {
        //             p
        //         } else {
        //             p.invert()
        //         }
        //     })
        //     .mute_background(None);
        // iced::widget::button::Appearance {
        //     background: style
        //         .options
        //         .background
        //         .then_some(palette.background.into()),
        //     border_radius: 3.0,
        //     border_width: if style.options.border { 1.0 } else { 0.0 },
        //     border_color: palette.text,
        //     text_color: palette.text,
        //     ..Default::default()
        // }
        todo!()
    }
}
