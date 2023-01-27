use bookmark_ui_util::{
    text_button::{Style as TbStyle, TextButton},
    IteratorWidgetExt,
};
use iced::{
    widget::{container, Column},
    Length, Sandbox, Settings,
};
use std::borrow::Cow;
use tap::Pipe;

struct App;

impl Sandbox for App {
    type Message = Cow<'static, str>;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        "Buttons".into()
    }

    fn update(&mut self, message: Self::Message) {
        println!("message: {message}")
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        const NAMES: &[(&str, TbStyle)] = &[
            ("One", TbStyle::Primary),
            ("Two", TbStyle::Secondary),
            ("Three", TbStyle::Positive),
            ("Four", TbStyle::Destructive),
        ];
        Column::new()
            .push(
                NAMES
                    .iter()
                    .collect_row(|(name, style)| TextButton::new(name).style(*style))
                    .spacing(3),
            )
            .push(
                NAMES
                    .iter()
                    .collect_row(|(name, style)| {
                        TextButton::new_with_on_press(name, || Cow::from(name.to_string()))
                            .style(*style)
                    })
                    .spacing(3),
            )
            .spacing(3)
            .pipe(container)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
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
