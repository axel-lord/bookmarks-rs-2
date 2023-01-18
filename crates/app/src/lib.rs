//! Application ui library.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use data::FileData;
use derive_more::From;
use iced::{
    executor, theme,
    widget::{button, container, scrollable, text, Column, Row},
    Command, Element, Length,
};
use std::path::PathBuf;
use tap::Pipe;

trait IterElements<Msg>: Iterator {
    fn collect_coumn<'a, E, F>(self, f: F) -> Column<'a, Msg>
    where
        E: Into<Element<'a, Msg>>,
        F: FnMut(Self::Item) -> E;

    fn collect_row<'a, E, F>(self, f: F) -> Row<'a, Msg>
    where
        E: Into<Element<'a, Msg>>,
        F: FnMut(Self::Item) -> E;
}

impl<I, Msg> IterElements<Msg> for I
where
    I: Iterator,
{
    fn collect_row<'a, E, F>(self, mut f: F) -> Row<'a, Msg>
    where
        E: Into<Element<'a, Msg>>,
        F: FnMut(Self::Item) -> E,
    {
        self.fold(Row::new(), |row, item| row.push(f(item)))
    }

    fn collect_coumn<'a, E, F>(self, mut f: F) -> Column<'a, Msg>
    where
        E: Into<Element<'a, Msg>>,
        F: FnMut(Self::Item) -> E,
    {
        self.fold(Column::new(), |column, item| column.push(f(item)))
    }
}

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
#[derive(Debug, Default)]
pub struct App {
    data: Option<FileData>,
}

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
    /// Signal a bookmark should be opened.
    #[from(ignore)]
    OpenBookmark(uuid::Uuid),
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
                self.data = Some(file_data);
                Command::none()
            }
            Message::FileLoaded(Err(err)) => {
                eprintln!("failed to load file data: {err}");
                Command::none()
            }
            Message::LoadFile(file) => {
                Command::perform(data::FileData::load(file), Message::FileLoaded)
            }
            Message::OpenBookmark(id) => {
                if let Some(ref file_data) = self.data {
                    if let Some(bookmark) = file_data
                        .bookmark
                        .iter()
                        .find(|bookmark| bookmark.uuid == id)
                    {
                        println!("Open \"{}\"", bookmark.url);
                    } else {
                        println!("could not find bookmark");
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let Some(ref file_data) = self.data else {
            return text("no data loaded").pipe(container).width(Length::Fill).height(Length::Fill).center_x().center_y().into();
        };

        file_data
            .bookmark
            .iter()
            .collect_coumn(|bookmark| {
                text(bookmark.url.clone())
                    .pipe(button)
                    .on_press(bookmark.uuid)
                    .style(theme::Button::Text)
                    .pipe(Element::from)
                    .map(Message::OpenBookmark)
            })
            .width(Length::Fill)
            .pipe(scrollable)
            .into()
    }
}
