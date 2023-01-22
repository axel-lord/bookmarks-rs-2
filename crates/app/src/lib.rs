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
use std::{marker::PhantomData, path::PathBuf};
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

struct Tabs<'a, 'b, State, OnChoice, Content, Message, Widget> {
    _lifetime: PhantomData<&'a (Message, Widget)>,
    tabs: Option<&'b [State]>,
    current: Option<usize>,
    on_choice: Option<OnChoice>,
    content: Option<Content>,
}

impl<'a, Message, State, OnChoice, Content, Widget> Default
    for Tabs<'a, '_, State, OnChoice, Content, Message, Widget>
{
    fn default() -> Self {
        Self {
            _lifetime: PhantomData::default(),
            tabs: None,
            current: None,
            on_choice: None,
            content: None,
        }
    }
}

impl<'a, 'b, Message, State, OnChoice, Content, Widget>
    Tabs<'a, 'b, State, OnChoice, Content, Message, Widget>
{
    fn new() -> Self {
        Self::default()
    }

    fn current(self, current: usize) -> Self {
        Self {
            current: Some(current),
            ..self
        }
    }

    fn tabs(self, tabs: &'b [State]) -> Self
    where
        State: ToString,
    {
        Self {
            tabs: Some(tabs),
            ..self
        }
    }

    fn on_choice(self, on_choice: OnChoice) -> Self
    where
        OnChoice: 'a + Clone + Fn(usize) -> Message,
    {
        Self {
            on_choice: Some(on_choice),
            ..self
        }
    }

    fn content(self, content: Content) -> Self
    where
        Content: FnMut(&State) -> Widget,
    {
        Self {
            content: Some(content),
            ..self
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
    fn from(value: Tabs<'a, '_, State, OnChoice, Content, Message, Widget>) -> Self {
        let Some(tabs) = value.tabs else {
            panic!("no tabs given to Tabs")
        };
        let Some(current) = value.current else {
            panic!("not current given to Tabs")
        };
        let Some(on_choice) = value.on_choice else {
            panic!("no on_choice given to Tabs")
        };
        let Some(mut content) = value.content else {
            panic!("no content function given to Tabs")
        };
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
                    .map(on_choice.clone())
            }))
            .push(content(&tabs[current]))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
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

        Tabs::new()
            .on_choice(Message::SelTab)
            .current(self.selected_tab)
            .tabs(&self.tabs)
            .content(|tab_state| match tab_state.as_str() {
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
            })
            .into()
    }
}
