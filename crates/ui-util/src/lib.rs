//! Some utilities to make working with ui easier.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use color::{ColorManipExt, ContrastPalette};
use iced::{
    widget::{Column, Row},
    Element,
};

pub mod color;
pub mod tabs;
pub mod text_button;
pub mod theme;

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
    /// Get [`ContrastPalette`] representing current theme.
    #[must_use]
    pub fn contrast_palette(&self) -> ContrastPalette {
        ContrastPalette::monochrome()
    }
}

/// Configuration for boxes.
#[derive(Clone, Copy, Debug, Default)]
pub struct BoxOptions {
    /// Whether to show a border, only applies if possible.
    pub show_border: bool,
    /// Whether to set text color, only applies if possible.
    pub update_text: bool,
    /// Whether to set background color, only applies if possible.
    pub opaque_background: bool,
}

impl BoxOptions {
    /// Construct with border set to passed value and the other fields default constructed.
    #[must_use]
    pub fn with_border(border: bool) -> Self {
        Self {
            show_border: border,
            ..Default::default()
        }
    }

    /// Construct with text set to passed value and the other fields default constructed.
    #[must_use]
    pub fn with_text(text: bool) -> Self {
        Self {
            update_text: text,
            ..Default::default()
        }
    }

    /// Construct with background set to passed value and the other fields default constructed.
    #[must_use]
    pub fn with_background(background: bool) -> Self {
        Self {
            opaque_background: background,
            ..Default::default()
        }
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
                    Theme::DarkMute => palette.dim.mute(None),
                },
                theme::Text::Color(color) => color,
            }),
        }
    }
}

impl iced::widget::container::StyleSheet for Theme {
    type Style = theme::Container;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
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

    fn active(&self, _style: &Self::Style, _is_active: bool) -> iced::widget::toggler::Appearance {
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

    fn hovered(&self, _style: &Self::Style, _is_active: bool) -> iced::widget::toggler::Appearance {
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

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
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

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
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
