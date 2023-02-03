//! Different themes for widgets.

use crate::{ContrastPalette, Theme};
use bookmark_util::Somewhere;
use derivative::Derivative;
use iced::Color;

/// Enum used to determine what Theme setting style with the value theme is using.
#[derive(Clone, Copy, Debug, Default)]
pub enum Var {
    /// The standard variant is to be used.
    #[default]
    Std,
    /// The alternate variant is to be used.
    Alt,
}

/// Style used fo [Application][iced::application::Application].
#[derive(Default, Debug, Clone)]
pub enum Application {
    /// Use the default style of the current theme.
    #[default]
    Theme,
    /// Use a palette based on contrasting colors swapping which is used for what, and modifying
    /// them based on theme.
    ContrastPalette(ContrastPalette),
    /// Set colors manually.
    Manual {
        /// Background color in use by application.
        background: Color,
        /// Text color in use by application.
        text: Color,
    },
    /// Implement style yourself.
    Custom(Somewhere<dyn iced::application::StyleSheet<Style = Theme>>),
}

/// Style used for [Container][iced::widget::Container] widgets
#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub enum Container {
    /// Use the default style of the current theme.
    #[derivative(Default)]
    Theme(Var),
    /// Use a palette based on contrast swapping what is foreground and background based on
    /// theme
    ContrastPalette {
        /// [ContrastPalette] to use.
        palette: ContrastPalette,
        /// If the background should be shown.
        opaque_background: bool,
        /// If the border should be shown.
        show_border: bool,
        /// If the text color should be changed.
        update_text: bool,
    },
    /// Use the provided colors for theme, they are used as is and None values indicate they are
    /// not present (no background, no text update, no border)
    Manual {
        /// If the background should be visible and if so what color.
        background: Option<Color>,
        /// If the text color should be changed and if so to what color.
        text: Option<Color>,
        /// If the border should be shown and if so what color.
        border: Option<Color>,
    },
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

/// Style used for [Toggler][iced::widget::Toggler] widgets.
#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub enum Toggler {
    /// Use the default style of the current theme.
    #[derivative(Default)]
    Theme(Var),
    /// Implement the style yourself.
    Custom(Somewhere<dyn iced::widget::toggler::StyleSheet<Style = Theme>>),
}

/// Style used for [Button][iced::widget::Button] widgets.
#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub enum Button {
    /// Use the default style of the current theme.
    #[derivative(Default)]
    Theme(Var),
    /// Implement the style yourself.
    Custom(Somewhere<dyn iced::widget::button::StyleSheet<Style = Theme>>),
}
