//! Module for [`TextButton`] widget builder.

use std::marker::PhantomData;

use bookmark_util::AnyWithExt;
use iced::{
    widget::{button, text, Button},
    Element, Length,
};
use tap::Pipe;

/// A widget for buttons with text content and messages not needing clone implementations.
#[derive(Debug)]
pub struct TextButton<'a, Message, OnPress = ()> {
    _lifetime: PhantomData<&'a Message>,
    content: String,
    on_press: Option<OnPress>,
    width: Option<Length>,
    height: Option<Length>,
}

impl<Message, OnPress> Default for TextButton<'_, Message, OnPress> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData::default(),
            content: String::default(),
            on_press: None,
            width: None,
            height: None,
        }
    }
}

impl<Message, OnPress> TextButton<'_, Message, OnPress>
where
    OnPress: 'static + Fn() -> Message,
{
    /// Create a new [`TextButton`] with given content and on press message factory.
    #[must_use]
    pub fn new(content: &impl ToString, on_press: OnPress) -> Self {
        Self {
            _lifetime: PhantomData::default(),
            content: content.to_string(),
            on_press: Some(on_press),
            ..TextButton::default()
        }
    }
}

impl<Message> TextButton<'_, Message> {
    /// Create a new [`TextButton`] with given content and no action on press.
    #[must_use]
    pub fn new(content: &impl ToString) -> Self {
        Self {
            _lifetime: PhantomData::default(),
            content: content.to_string(),
            on_press: None,
            ..TextButton::default()
        }
    }
}

impl<'a, Message, OnPress> TextButton<'a, Message, OnPress> {
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

    fn button(content: String, width: Option<Length>, height: Option<Length>) -> Button<'a, ()> {
        button(text(content))
            .with(width, Button::width)
            .with(height, Button::height)
    }
}

impl<'a, Message, OnPress> From<TextButton<'a, Message, OnPress>> for Element<'a, Message>
where
    OnPress: 'static + Fn() -> Message,
{
    fn from(value: TextButton<'a, Message, OnPress>) -> Self {
        let TextButton {
            content, on_press: Some(on_press), width, height,..
        } = value else {
            panic!(concat!(
                "when a bookmark_ui_util::button::Button has a <Fn() -> Message> ",
                "OnPress generic parameter the on_press field should always have ",
                "a value specified",
            ));
        };
        TextButton::<Message>::button(content, width, height)
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
            ..
        } = value;
        TextButton::<Message>::button(content, width, height)
            .pipe(Element::from)
            .map(|_: ()| {
                unimplemented!(concat!(
                    "this message should never be fired since ",
                    "on_press has not been called for the button",
                ))
            })
    }
}
