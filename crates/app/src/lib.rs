//! Application ui library.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use bookmark_data::FileData;
use bookmark_ui_util::IteratorWidgetExt;
use derive_more::From;
use iced::{
    executor, theme,
    widget::{button, container, scrollable, text, Column},
    Command, Element, Length,
};
use std::path::PathBuf;
use tap::Pipe;

pub use iced::Application;

/// Application class.
#[derive(Debug, Default)]
pub struct App {
    data: Option<FileData>,
    tabs: Vec<String>,
    selected_tab: usize,
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
    FileLoaded(bookmark_data::Result<FileData>),
    /// Signal a file should be loaded.
    #[from(ignore)]
    LoadFile(PathBuf),
    /// Signal a bookmark should be opened.
    #[from(ignore)]
    OpenBookmark(uuid::Uuid),
    /// Add a blank tab.
    #[from(ignore)]
    AddTab(String),
    /// Select a blank tab.
    #[from(ignore)]
    SelTab(usize),
}

fn tabs<'a, M, S, F, E>(
    tabs: &[S],
    current: usize,
    on_choice: fn(usize) -> M,
    mut content: F,
) -> Element<'a, M>
where
    S: ToString,
    M: 'a,
    F: FnMut(&S) -> E,
    E: Into<Element<'a, M>>,
{
    assert!((0..tabs.len()).contains(&current));
    Column::new()
        .push(tabs.iter().enumerate().collect_row(|(index, tab)| {
            tab.to_string()
                .pipe(text)
                .pipe(button)
                .pipe(|btn| {
                    if index == current {
                        btn
                    } else {
                        btn.on_press(index)
                    }
                })
                .pipe(Element::from)
                .map(on_choice)
        }))
        .push(content(&tabs[current]))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                tabs: ["bookmarks", "edit", "log"].map(String::from).into(),
                ..Self::default()
            },
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
            Message::LoadFile(file) => Command::perform(FileData::load(file), Message::FileLoaded),
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
            Message::AddTab(name) => {
                self.tabs.push(name);
                Command::none()
            }
            Message::SelTab(tab) => {
                self.selected_tab = tab;
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let Some(ref file_data) = self.data else {
            return text("no data loaded").pipe(container).width(Length::Fill).height(Length::Fill).center_x().center_y().into();
        };

        tabs(
            &self.tabs,
            self.selected_tab,
            Message::SelTab,
            |tab_state| match tab_state.as_str() {
                "bookmarks" => file_data
                    .bookmark
                    .iter()
                    .take(128)
                    .collect_column(|bookmark| {
                        text(bookmark.url.clone())
                            .pipe(button)
                            .on_press(bookmark.uuid)
                            .style(theme::Button::Text)
                            .pipe(Element::from)
                            .map(Message::OpenBookmark)
                    })
                    .width(Length::Fill)
                    .pipe(scrollable)
                    .pipe(Element::from),
                _ => text("no content").into(),
            },
        )
    }
}
