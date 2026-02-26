mod update;
mod view;

use iced::{Subscription, Task};
use std::path::PathBuf;
use crate::{install, net};

#[derive(Debug, Clone)]
pub enum Msg {
    GotRelease(Result<net::github::Release, String>),
    GoPath,
    GoBack,
    PathChanged(String),
    Browse,
    BrowsePicked(Option<PathBuf>),
    Reinstall,
    StartInstall,
    DlDone(Result<PathBuf, String>),
    InstallDone(Result<(), String>),
    Uninstall,
    UninstallDone(Result<(), String>),
    Tick,
    Close,
}

pub enum Page {
    Loading,
    Welcome,
    Found,
    Path,
    Installing,
    Done { title: String, subtitle: String },
    Error(String),
}

pub struct App {
    pub page: Page,
    pub version: String,
    pub release: Option<net::github::Release>,
    pub path: String,
    pub status: String,
    pub pct: f32,
    pub target_pct: f32,
    pub animating: bool,
    pub done_pending: Option<(String, String)>,
}

impl App {
    pub fn new() -> (Self, Task<Msg>) {
        let app = Self {
            page: Page::Loading,
            version: String::new(),
            release: None,
            path: install::default_path().to_string_lossy().to_string(),
            status: String::from("fetching lst ver..."),
            pct: 0.0,
            target_pct: 0.0,
            animating: false,
            done_pending: None,
        };
        let task = Task::perform(net::github::latest(), Msg::GotRelease);
        (app, task)
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        if self.animating {
            iced::time::every(std::time::Duration::from_millis(16)).map(|_| Msg::Tick)
        } else {
            Subscription::none()
        }
    }
}
