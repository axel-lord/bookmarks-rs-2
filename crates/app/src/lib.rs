//! Application ui library.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use derive_more::From;
use iced::{
    executor, theme,
    widget::{container, text},
    Command, Length,
};
use std::path::PathBuf;
use tap::Pipe;

#[allow(clippy::module_name_repetitions)]
mod data {
    use serde::{Deserialize, Serialize};
    use std::{io, path::PathBuf, result};
    use tap::Pipe;
    use thiserror::Error;
    use tokio::fs;
    use uuid::Uuid;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error(transparent)]
        IO(#[from] io::Error),
        #[error(transparent)]
        RmpDeserialize(#[from] rmp_serde::decode::Error),
    }

    pub type Result<T = ()> = result::Result<T, Error>;

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct FileData {
        pub tag: Vec<String>,
        pub category: Vec<CategoryData>,
        pub bookmark: Vec<BookmarkData>,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct CategoryData {
        pub name: String,
        pub info: String,
        pub identifier: IdentifierData,
        pub subcategory: Vec<CategoryData>,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct IdentifierData {
        pub require: Vec<String>,
        pub whole: Vec<String>,
        pub include: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BookmarkData {
        pub url: String,
        pub info: String,
        pub uuid: Uuid,
        pub tag: Vec<String>,
    }

    impl FileData {
        pub async fn load(path: PathBuf) -> Result<Self> {
            Ok(fs::read(path).await?.pipe_deref(rmp_serde::from_slice)?)
        }
    }

    impl Default for BookmarkData {
        fn default() -> Self {
            Self {
                url: String::new(),
                info: String::new(),
                uuid: Uuid::new_v4(),
                tag: Vec::new(),
            }
        }
    }
}

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
#[derive(Debug, From)]
pub enum Message {
    /// Signal a file has been loaded.
    FileLoaded(data::Result<data::FileData>),
    /// Signal a file should be loaded.
    #[from(ignore)]
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
            Message::FileLoaded(Ok(file_data)) => {
                dbg!(file_data.category);
                Command::none()
            }
            Message::FileLoaded(Err(err)) => {
                eprintln!("failed to load file data: {err}");
                Command::none()
            }
            Message::LoadFile(file) => {
                Command::perform(data::FileData::load(file), Message::FileLoaded)
            }
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
