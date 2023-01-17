use bookmark_app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    App::run(Settings {
        ..Default::default()
    })
}
