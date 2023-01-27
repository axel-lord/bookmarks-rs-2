//! Module for [`TextButton`] widget builder.

use bookmark_util::AnyWithExt;
use derivative::Derivative;
use iced::{
    theme,
    widget::{button, text, Button},
    Element, Length, Padding,
};
use std::marker::PhantomData;
use tap::Pipe;

/// A widget for buttons with text content and messages not needing clone implementations.
#[derive(Debug, Derivative)]
#[derivative(Default(bound = ""))]
pub struct TextButton<'a, Message, OnPress = ()> {
    _lifetime: PhantomData<&'a Message>,
    content: String,
    on_press: Option<OnPress>,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    style: Style,
}

/// Style used by [`TextButton`].
#[derive(Clone, Copy, Debug, Default)]
pub enum Style {
    /// The primary look for buttons.
    #[default]
    Primary,
    /// A Secondary look for buttons.
    Secondary,
    /// A look for buttons with "positive" actions.
    Positive,
    /// A look for buttons with "negative" actions.
    Destructive,
}

impl<Message> TextButton<'_, Message, ()> {
    /// Create a new [`TextButton`] with given content and no action on press.
    #[must_use]
    pub fn new(content: &impl ToString) -> Self {
        Self {
            content: content.to_string(),
            ..TextButton::default()
        }
    }
}

impl<'a, Message, OnPress> TextButton<'a, Message, OnPress> {
    /// Create a new [`TextButton`] with given content and on press message factory.
    #[must_use]
    pub fn new_with_on_press(content: &impl ToString, on_press: OnPress) -> Self
    where
        OnPress: 'static + Fn() -> Message,
    {
        Self {
            content: content.to_string(),
            on_press: Some(on_press),
            ..TextButton::default()
        }
    }
    /// Sets the width of the [`TextButton`].
    #[must_use]
    pub fn width(self, width: Length) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }
    /// Sets the height of the [`TextButton`].
    #[must_use]
    pub fn height(self, height: Length) -> Self {
        Self {
            height: Some(height),
            ..self
        }
    }
    /// Sets the padding of the [`TextButton`].
    #[must_use]
    pub fn padding(self, padding: impl Into<Padding>) -> Self {
        Self {
            padding: Some(padding.into()),
            ..self
        }
    }

    /// Sets the style in use by the button.
    #[must_use]
    pub fn style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    fn button(
        content: String,
        width: Option<Length>,
        height: Option<Length>,
        padding: Option<Padding>,
        style: Style,
    ) -> Button<'a, ()> {
        button(text(content))
            .with(width, Button::width)
            .with(height, Button::height)
            .padding(padding.unwrap_or(Padding::from(3)))
            .style(theme::Button::Custom(Box::new(style)))
    }
}

impl<'a, Message, OnPress> From<TextButton<'a, Message, OnPress>> for Element<'a, Message>
where
    OnPress: 'static + Fn() -> Message,
{
    fn from(value: TextButton<'a, Message, OnPress>) -> Self {
        let TextButton {
            content, on_press: Some(on_press), width, height,padding,style,..
        } = value else {
            panic!(concat!(
                "when a bookmark_ui_util::button::Button has a <Fn() -> Message> ",
                "OnPress generic parameter the on_press field should always have ",
                "a value specified",
            ));
        };
        TextButton::<Message>::button(content, width, height, padding, style)
            .on_press(())
            .pipe(Element::from)
            .map(move |_: ()| on_press())
    }
}

impl<'a, Message> From<TextButton<'a, Message>> for Element<'a, Message> {
    fn from(value: TextButton<'a, Message>) -> Self {
        let TextButton {
            content,
            width,
            height,
            padding,
            style,
            ..
        } = value;
        TextButton::<Message>::button(content, width, height, padding, style)
            .pipe(Element::from)
            .map(|_: ()| {
                unimplemented!(concat!(
                    "this message should never be fired since ",
                    "on_press has not been called for the button",
                ))
            })
    }
}

macro_rules! button_appearance {
    ($kind:ident, $strength:ident, $palette:expr) => {{
        button::Appearance {
            background: Some($palette.$kind.$strength.color.into()),
            text_color: $palette.primary.strong.text,
            ..Default::default()
        }
    }};
}

macro_rules! button_style_impl {
    ($ty:ty, {$($arm:pat => $kind:ident),+ $(,)?}) => {
        impl button::StyleSheet for $ty {
            type Style = iced::Theme;

            fn active(&self, style: &Self::Style) -> button::Appearance {
                let palette = style.extended_palette();
                match self {
                    $(
                        $arm => button_appearance!($kind, strong, palette),
                    )*
                }
            }

            fn pressed(&self, style: &Self::Style) -> button::Appearance {
                let palette = style.extended_palette();
                match self {
                    $(
                        $arm => button_appearance!($kind, weak, palette),
                    )*
                }
            }

            fn hovered(&self, style: &Self::Style) -> button::Appearance {
                let palette = style.extended_palette();
                match self {
                    $(
                        $arm => button_appearance!($kind, base, palette),
                    )*
                }
            }
        }
    };
}

button_style_impl!(Style, {
    Style::Primary => primary,
    Style::Secondary => secondary,
    Style::Positive => success,
    Style::Destructive => danger,
});
