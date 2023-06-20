extern crate core;
#[macro_use]
extern crate log;

use iced::{Application, Settings};

use crate::capture::ScreenCapture;
use crate::gui::app::App;
use crate::output::OutputSink;
use crate::result::Result;

mod capture;
mod config;
mod encoder;
mod gui;
mod inputs;
mod output;
mod performance_profiler;
mod result;
mod signaller;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    App::run(Settings {
        window: iced::window::Settings {
            size: (640, 373),
            min_size: Some((400, 300)),
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
}
