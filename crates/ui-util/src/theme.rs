//! Different themes for widgets.

use crate::{BoxOptions, ContrastPalette, Theme};
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
    /// Use the provided colors for theme, they are used as is and None values indicate they are
    /// not present (no background, no text update, no border)
    Manual {
        background: Option<Color>,
        text: Option<Color>,
        border: Option<Color>,
    },
    /// Use a palette where the foreground and background values are set.
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
