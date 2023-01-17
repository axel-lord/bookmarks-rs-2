use std::path::PathBuf;

use bookmark_app::{App, Flags};
use clap::Parser;
use iced::{Application, Settings};

#[derive(Parser)]
struct Cli {
    files: Vec<PathBuf>,
}

impl From<Cli> for Flags {
    fn from(value: Cli) -> Self {
        Self { files: value.files }
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        flags: Cli::parse().into(),
        ..Default::default()
    })
}
