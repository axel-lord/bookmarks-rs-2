//! Application ui library.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use iced::{
    executor, theme,
    widget::{container, text},
    Command, Length,
};
use tap::Pipe;

pub use iced::Application;

/// Application class.
#[derive(Clone, Copy, Debug, Default)]
pub struct App;

impl Application for App {
    type Executor = executor::Default;

    type Message = ();

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Application".into()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text("Message")
            .pipe(container)
            .padding(3)
            .style(theme::Container::Box)
            .pipe(container)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
