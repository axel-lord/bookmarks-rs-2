//! Module for [Tabs] widget.

use crate::IteratorWidgetExt;
use iced::{
    alignment::Horizontal,
    widget::{button, container, text, Column},
    Element, Length,
};
use std::marker::PhantomData;
use tap::Pipe;

/// A Widget representing a tab view.
pub struct Tabs<'a, 'b, State, OnChoice, Content, Message, Widget> {
    _lifetime: PhantomData<&'a (Message, Widget)>,
    tabs: &'b [State],
    current: usize,
    on_choice: OnChoice,
    content: Content,
    horizontal: bool,
}

impl<'a, 'b, Message, State, OnChoice, Content, Widget>
    Tabs<'a, 'b, State, OnChoice, Content, Message, Widget>
where
    State: ToString,
    OnChoice: 'a + Clone + Fn(usize) -> Message,
    Content: FnMut(&State) -> Widget,
    Message: 'a,
    Widget: Into<Element<'a, Message>>,
{
    /// Construct a new [Tabs] with passed arguments and functions to determine state.
    ///
    /// # Panics
    /// If current is not an index of tabs.
    pub fn new(tabs: &'b [State], current: usize, on_choice: OnChoice, content: Content) -> Self {
        assert!((0..tabs.len()).contains(&current));
        Self {
            _lifetime: PhantomData::default(),
            tabs,
            current,
            on_choice,
            content,
            horizontal: false,
        }
    }
}

impl<'a, Message, State, OnChoice, Content, Widget>
    From<Tabs<'a, '_, State, OnChoice, Content, Message, Widget>> for Element<'a, Message>
where
    State: ToString,
    Message: 'a,
    Widget: Into<Element<'a, Message>>,
    OnChoice: 'a + Clone + Fn(usize) -> Message,
    Content: FnMut(&State) -> Widget,
{
    fn from(mut value: Tabs<'a, '_, State, OnChoice, Content, Message, Widget>) -> Self {
        if value.horizontal {
            todo!()
        } else {
            Column::new()
                .push(value.tabs.iter().enumerate().collect_row(|(index, tab)| {
                    tab.to_string()
                        .pipe(text)
                        .horizontal_alignment(Horizontal::Center)
                        .width(Length::Fill)
                        .pipe(button)
                        .pipe(|btn| {
                            if index == value.current {
                                btn
                            } else {
                                btn.on_press(index)
                            }
                        })
                        .style(style::Tab::build())
                        .width(Length::Fill)
                        .pipe(container)
                        .width(Length::Fill)
                        .max_width(150)
                        .pipe(Element::from)
                        .map(value.on_choice.clone())
                }))
                .push((value.content)(&value.tabs[value.current]))
                .width(Length::Fill)
                .height(Length::Fill)
                .pipe(container)
                .style(style::Content::build())
                .into()
        }
    }
}

mod style {
    use iced::{
        theme,
        widget::{button, container},
        Theme,
    };

    pub struct Tab;

    impl Tab {
        pub fn build() -> theme::Button {
            theme::Button::Custom(Box::new(Self))
        }
    }

    impl button::StyleSheet for Tab {
        type Style = Theme;

        fn active(&self, style: &Self::Style) -> button::Appearance {
            let palette = style.extended_palette();

            button::Appearance {
                background: Some(palette.background.strong.color.into()),
                border_radius: 0.0,
                border_width: 0.0,
                text_color: palette.background.weak.text,
                ..Default::default()
            }
        }

        fn disabled(&self, style: &Self::Style) -> button::Appearance {
            let palette = style.extended_palette();

            button::Appearance {
                background: Some(palette.background.base.color.into()),
                border_radius: 0.0,
                border_width: 0.0,
                text_color: palette.background.strong.text,
                ..Default::default()
            }
        }

        fn hovered(&self, style: &Self::Style) -> button::Appearance {
            let palette = style.extended_palette();

            button::Appearance {
                background: Some(palette.background.weak.color.into()),
                border_radius: 0.0,
                border_width: 0.0,
                text_color: palette.background.strong.text,
                ..Default::default()
            }
        }

        fn pressed(&self, style: &Self::Style) -> button::Appearance {
            self.disabled(style)
        }
    }

    pub struct Content;

    impl Content {
        pub fn build() -> theme::Container {
            theme::Container::Custom(Box::new(Self))
        }
    }

    impl container::StyleSheet for Content {
        type Style = Theme;

        fn appearance(&self, style: &Self::Style) -> container::Appearance {
            let palette = style.extended_palette();

            container::Appearance {
                text_color: Some(palette.background.base.text),
                background: Some(palette.background.base.color.into()),
                border_radius: 0.0,
                border_width: 0.0,
                ..Default::default()
            }
        }
    }
}
