use bookmark_ui_util::Theme;
use iced::{widget::text, Application, Element, Settings};
use std::borrow::Cow;

struct App;

impl Application for App {
    type Message = Cow<'static, str>;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        "Buttons".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        println!("message: {message}");
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
        text("placeholder").into()
    }

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self, iced::Command::none())
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            size: (250, 100),
            ..Default::default()
        },
        ..Default::default()
    })
}
