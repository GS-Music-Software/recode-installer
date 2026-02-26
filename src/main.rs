#![windows_subsystem = "windows"]

mod app;
mod consts;
mod gui;
mod net;
mod install;

use iced::{Theme, Size};

fn main() -> iced::Result {
    iced::application(consts::TITLE, app::App::update, app::App::view)
        .theme(|_| Theme::Dark)
        .window_size(Size::new(consts::WIN_W, consts::WIN_H))
        .resizable(false)
        .subscription(app::App::subscription)
        .run_with(app::App::new)
}
