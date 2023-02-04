//! Some utilities to make working with ui easier.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use color::{ColorManipExt, ContrastPalette, Palette, ThemePalette};
use iced::{
    widget::{Column, Row},
    Background, Color, Element,
};
use theme::Var;

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
    /// Get [`ContrastPalette`] representing current theme base.
    #[must_use]
    pub fn contrast_palette(&self) -> ContrastPalette {
        ContrastPalette::monochrome()
    }

    /// Get a [`ContrastPalette`] representing current theme alt.
    #[must_use]
    pub fn contrast_palette_alt(&self) -> ContrastPalette {
        ContrastPalette {
            bright: Color::from_rgb8(150, 200, 255),
            dim: Color::from_rgb8(0, 0, 40),
        }
    }
    /// Get a [`ThemePalette`] representing the current theme.
    #[must_use]
    pub fn theme_palette(&self) -> ThemePalette {
        ThemePalette {
            mute: self.convert_palette(self.contrast_palette().mute_dim(None)),
            alt: self.convert_palette(self.contrast_palette_alt().mute_dim(None)),
            mute_highlight: self.convert_palette(self.contrast_palette()),
            alt_highlight: self.convert_palette(self.contrast_palette_alt()),
        }
    }

    /// Get a [Palette] from a [`ContrastPalette`] using current theme.
    #[must_use]
    pub fn convert_palette(&self, ContrastPalette { bright, dim }: ContrastPalette) -> Palette {
        match self {
            Theme::Light => Palette {
                border: dim,
                background: bright,
                foreground: dim,
                text: dim,
            },
            Theme::Dark => Palette {
                border: bright,
                background: dim,
                foreground: bright,
                text: bright,
            },
            Theme::DarkMute => Palette {
                border: bright,
                background: dim.mute(None),
                foreground: bright,
                text: bright,
            },
        }
    }

    /// Get the border radius in use.
    #[must_use]
    pub fn border_radius(&self) -> f32 {
        0.0
    }
}

impl iced::application::StyleSheet for Theme {
    type Style = theme::Application;

    fn appearance(&self, style: &Self::Style) -> iced::application::Appearance {
        use iced::application::Appearance;
        match style {
            theme::Application::Theme => {
                let Palette {
                    background, text, ..
                } = self.theme_palette().mute;
                Appearance {
                    background_color: background,
                    text_color: text,
                }
            }
            theme::Application::ContrastPalette(palette) => {
                let Palette {
                    background, text, ..
                } = self.convert_palette(palette.mute_dim(None));
                Appearance {
                    background_color: background,
                    text_color: text,
                }
            }
            theme::Application::Custom(style) => style.appearance(self),
        }
    }
}

impl iced::widget::text::StyleSheet for Theme {
    type Style = Option<theme::Text>;

    fn appearance(&self, style: Self::Style) -> iced::widget::text::Appearance {
        use iced::widget::text::Appearance;
        Appearance {
            color: style.map(|style| match style {
                theme::Text::Theme => self.theme_palette().mute.text,
                theme::Text::ContrastPalette(palette) => {
                    self.convert_palette(palette.mute_dim(None)).text
                }
                theme::Text::Color(color) => color,
            }),
        }
    }
}

impl iced::widget::container::StyleSheet for Theme {
    type Style = Option<theme::Container>;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        use iced::widget::container::Appearance;
        style.as_ref().map_or(
            Appearance {
                text_color: None,
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            |style| match style {
                theme::Container::Theme(Var::Alt) => {
                    let Palette {
                        border,
                        background,
                        text,
                        ..
                    } = self.theme_palette().alt;
                    Appearance {
                        text_color: Some(text),
                        background: Some(background.into()),
                        border_radius: self.border_radius(),
                        border_width: 1.0,
                        border_color: border,
                    }
                }
                theme::Container::Theme(Var::Std) => {
                    let Palette {
                        background,
                        foreground,
                        ..
                    } = self.theme_palette().alt;
                    Appearance {
                        text_color: Some(background),
                        background: Some(foreground.into()),
                        border_radius: self.border_radius(),
                        border_width: 0.0,
                        border_color: Color::BLACK,
                    }
                }
                theme::Container::ContrastPalette(palette, var) => {
                    let Palette {
                        background,
                        text,
                        border,
                        ..
                    } = self.convert_palette(palette.mute_dim(None));
                    Appearance {
                        text_color: Some(text),
                        background: Some(background.into()),
                        border_radius: self.border_radius(),
                        border_width: if matches!(var, Var::Std) { 0.0 } else { 1.0 },
                        border_color: border,
                    }
                }
                theme::Container::Custom(custom) => custom.appearance(self),
            },
        )
    }
}

fn toggler_appearance(
    Palette {
        background,
        foreground,
        ..
    }: Palette,
) -> iced::widget::toggler::Appearance {
    iced::widget::toggler::Appearance {
        background: foreground,
        background_border: None,
        foreground: background,
        foreground_border: Some(foreground),
    }
}

fn toggler_alt_appearance(
    Palette {
        background,
        foreground,
        ..
    }: Palette,
) -> iced::widget::toggler::Appearance {
    iced::widget::toggler::Appearance {
        background,
        background_border: Some(foreground),
        foreground,
        foreground_border: Some(background),
    }
}

impl iced::widget::toggler::StyleSheet for Theme {
    type Style = theme::Toggler;

    fn active(&self, style: &Self::Style, is_active: bool) -> iced::widget::toggler::Appearance {
        match style {
            theme::Toggler::Custom(style_sheet) => style_sheet.active(self, is_active),
            theme::Toggler::Theme(Var::Std) => toggler_appearance(self.theme_palette().mute),
            theme::Toggler::Theme(Var::Alt) => toggler_alt_appearance(self.theme_palette().mute),
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> iced::widget::toggler::Appearance {
        match style {
            theme::Toggler::Custom(style_sheet) => style_sheet.hovered(self, is_active),
            theme::Toggler::Theme(Var::Std) => {
                toggler_appearance(self.theme_palette().mute_highlight)
            }
            theme::Toggler::Theme(Var::Alt) => {
                toggler_alt_appearance(self.theme_palette().mute_highlight)
            }
        }
    }
}

fn button_appearance(
    Palette {
        background,
        foreground,
        ..
    }: Palette,
    border_radius: f32,
) -> iced::widget::button::Appearance {
    iced::widget::button::Appearance {
        background: Some(foreground).map(Background::from),
        border_radius,
        border_width: 0.0,
        border_color: Color::BLACK,
        text_color: background,
        ..Default::default()
    }
}

fn button_alt_appearance(
    Palette {
        background,
        foreground,
        text,
        ..
    }: Palette,
    border_radius: f32,
) -> iced::widget::button::Appearance {
    iced::widget::button::Appearance {
        background: Some(background).map(Background::from),
        border_radius,
        border_width: 1.0,
        border_color: foreground,
        text_color: text,
        ..Default::default()
    }
}

impl iced::widget::button::StyleSheet for Theme {
    type Style = theme::Button;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match style {
            theme::Button::Custom(style_sheet) => style_sheet.active(self),
            theme::Button::Theme(Var::Std) => {
                button_appearance(self.theme_palette().mute, self.border_radius())
            }
            theme::Button::Theme(Var::Alt) => {
                button_alt_appearance(self.theme_palette().mute, self.border_radius())
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match style {
            theme::Button::Custom(style_sheet) => style_sheet.hovered(self),
            theme::Button::Theme(Var::Std) => {
                button_appearance(self.theme_palette().mute_highlight, self.border_radius())
            }
            theme::Button::Theme(Var::Alt) => {
                button_alt_appearance(self.theme_palette().mute_highlight, self.border_radius())
            }
        }
    }

    fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match style {
            theme::Button::Custom(style_sheet) => style_sheet.pressed(self),
            theme::Button::Theme(Var::Std) => {
                button_appearance(self.theme_palette().mute, self.border_radius())
            }
            theme::Button::Theme(Var::Alt) => {
                button_alt_appearance(self.theme_palette().mute, self.border_radius())
            }
        }
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match style {
            theme::Button::Custom(style_sheet) => style_sheet.disabled(self),
            theme::Button::Theme(Var::Std) => {
                button_appearance(self.theme_palette().mute.mute(None), self.border_radius())
            }
            theme::Button::Theme(Var::Alt) => {
                button_alt_appearance(self.theme_palette().mute.mute(None), self.border_radius())
            }
        }
    }
}
