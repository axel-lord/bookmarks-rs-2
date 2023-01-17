//! Application ui library.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use std::path::PathBuf;

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

/// Flags used to set initial state of [App].
#[derive(Default)]
pub struct Flags {
    /// Files to load on startup.
    pub files: Vec<PathBuf>,
}

/// Top Message class used by [App].
#[derive(Debug)]
pub enum Message {
    /// Signal a file has been loaded.
    FileLoaded(PathBuf),
    /// Signal a file should be loaded.
    LoadFile(PathBuf),
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::default(),
            if flags.files.is_empty() {
                Command::none()
            } else {
                flags
                    .files
                    .iter()
                    .cloned()
                    .map(|file| Command::perform(async move { file }, Message::LoadFile))
                    .pipe(Command::batch)
            },
        )
    }

    fn title(&self) -> String {
        "Application".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::FileLoaded(file) => {
                println!("loaded file {}", file.display());
                Command::none()
            }
            Message::LoadFile(file) => Command::perform(async move { file }, Message::FileLoaded),
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
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
