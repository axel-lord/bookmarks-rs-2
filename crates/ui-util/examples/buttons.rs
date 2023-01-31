use bookmark_ui_util::{theme, Theme};
use iced::{
    widget::{button, container, text, toggler, Column, Row},
    Alignment, Application, Element, Length, Settings,
};
use std::borrow::Cow;
use tap::Pipe;

struct App {
    theme: Theme,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    Text(Cow<'static, str>),
    Theme(Theme),
}

impl Application for App {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        "Buttons".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Text(message) => println!("message: {message}"),
            Message::Theme(theme) => self.theme = theme,
        }
        iced::Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        // const NAMES: &[(&str, TbStyle)] = &[
        //     ("One", TbStyle::Primary),
        //     ("Two", TbStyle::Secondary),
        //     ("Three", TbStyle::Positive),
        //     ("Four", TbStyle::Destructive),
        // ];
        // Column::new()
        //     .push(
        //         NAMES
        //             .iter()
        //             .collect_row(|(name, style)| TextButton::new(name).style(*style))
        //             .spacing(3),
        //     )
        //     .push(
        //         NAMES
        //             .iter()
        //             .collect_row(|(name, style)| {
        //                 TextButton::new_with_on_press(name, || Cow::from(name.to_string()))
        //                     .style(*style)
        //             })
        //             .spacing(3),
        //     )
        //     .spacing(3)
        //     .pipe(container)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .center_x()
        //     .center_y()
        //     .into()
        Column::new()
            .push(
                Row::new()
                    .spacing(3)
                    .align_items(Alignment::Center)
                    .push(text("use dark mode"))
                    .push(
                        toggler(None, matches!(self.theme, Theme::Dark), |b| {
                            Message::Theme(if b { Theme::Dark } else { Theme::Light })
                        })
                        .width(Length::Shrink)
                        .style(theme::Container::Theme),
                    ),
            )
            .push(
                Row::new()
                    .spacing(3)
                    .align_items(Alignment::Center)
                    .push(text("use dark mute mode"))
                    .push(
                        toggler(None, matches!(self.theme, Theme::DarkMute), |b| {
                            Message::Theme(if b { Theme::DarkMute } else { Theme::Light })
                        })
                        .width(Length::Shrink)
                        .style(theme::Container::Theme),
                    ),
            )
            .push(
                Row::new()
                    .spacing(3)
                    .align_items(Alignment::Center)
                    .push(
                        button("One")
                            .style(theme::Container::Theme)
                            .padding(3)
                            .on_press(Message::Text("One".into())),
                    )
                    .push(
                        button("Two")
                            .style(theme::Container::Theme)
                            .padding(3)
                            .on_press(Message::Text("Two".into())),
                    )
                    .push(
                        button("Three")
                            .style(theme::Container::Theme)
                            .padding(3)
                            .on_press(Message::Text("Three".into())),
                    ),
            )
            .spacing(3)
            .align_items(Alignment::End)
            .pipe(container)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(3)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme
    }

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                theme: Theme::Light,
            },
            iced::Command::none(),
        )
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            size: (500, 100),
            ..Default::default()
        },
        ..Default::default()
    })
}
