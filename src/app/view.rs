use iced::Element;
use crate::gui;
use super::{App, Msg, Page};

impl App {
    pub fn view(&self) -> Element<'_, Msg> {
        match &self.page {
            Page::Loading => gui::progress::view(&self.status, 0.0),
            Page::Welcome => gui::welcome::view(&self.version, Msg::GoPath, Msg::Close),
            Page::Found => gui::found::view(Msg::Reinstall, Msg::Uninstall, Msg::Close),
            Page::Path => gui::path::view(
                &self.path,
                Msg::PathChanged,
                Msg::Browse,
                Msg::GoBack,
                Msg::StartInstall,
            ),
            Page::Installing => gui::progress::view(&self.status, self.pct),
            Page::Done { title, subtitle } => gui::done::view(title, subtitle, Msg::Close),
            Page::Error(e) => gui::error::view(e, Msg::Close),
        }
    }
}
