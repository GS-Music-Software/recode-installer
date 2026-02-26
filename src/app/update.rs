use iced::Task;
use std::path::PathBuf;
use crate::{consts, install, net};
use super::{App, Msg, Page};

impl App {
    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::GotRelease(Ok(rel)) => {
                self.version = rel.tag_name.clone();
                self.release = Some(rel);
                self.page = Page::Welcome;
            }
            Msg::GotRelease(Err(e)) => {
                self.page = Page::Error(e);
            }
            Msg::GoPath => {
                let dest = PathBuf::from(&self.path);
                if install::is_installed(&dest) {
                    self.page = Page::Found;
                } else {
                    self.page = Page::Path;
                }
            }
            Msg::GoBack => {
                self.page = Page::Welcome;
            }
            Msg::PathChanged(p) => {
                self.path = p;
            }
            Msg::Browse => {
                let start = PathBuf::from(&self.path);
                return Task::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .set_directory(&start)
                            .set_title("Choose Install Location")
                            .pick_folder()
                            .await
                            .map(|h| h.path().to_path_buf())
                    },
                    Msg::BrowsePicked,
                );
            }
            Msg::BrowsePicked(Some(p)) => {
                self.path = p.to_string_lossy().to_string();
            }
            Msg::BrowsePicked(None) => {}
            Msg::Reinstall => {
                self.page = Page::Path;
            }
            Msg::StartInstall => {
                self.page = Page::Installing;
                self.status = String::from("Downloading...");
                self.pct = 0.0;
                self.target_pct = 15.0;
                self.animating = true;
                self.done_pending = None;

                let Some(rel) = &self.release else {
                    self.page = Page::Error("no release data".into());
                    return Task::none();
                };
                let asset = match net::github::pick_asset(rel) {
                    Ok(a) => a.clone(),
                    Err(e) => {
                        self.page = Page::Error(e);
                        return Task::none();
                    }
                };

                let dest = std::env::temp_dir().join(&asset.name);
                let url = asset.browser_download_url;

                return Task::perform(
                    async move { net::dl::fetch(&url, &dest).await },
                    Msg::DlDone,
                );
            }
            Msg::DlDone(Ok(bin)) => {
                self.status = String::from("Installing...");
                self.target_pct = 90.0;
                self.animating = true;
                let dest = PathBuf::from(&self.path);
                let extras: Vec<_> = self.release.as_ref()
                    .map(|r| net::github::extra_assets(r).into_iter().cloned().collect())
                    .unwrap_or_default();
                return Task::perform(
                    async move {
                        let icon_tmp = std::env::temp_dir().join(consts::ICON_NAME);
                        let icon = net::dl::fetch(consts::ICON_URL, &icon_tmp).await.ok();

                        let mut extra_paths = Vec::new();
                        for asset in &extras {
                            let tmp = std::env::temp_dir().join(&asset.name);
                            if let Ok(p) = net::dl::fetch(&asset.browser_download_url, &tmp).await {
                                extra_paths.push(p);
                            }
                        }

                        tokio::task::spawn_blocking(move || {
                            install::run(&bin, &dest, icon.as_deref(), &extra_paths)
                        })
                        .await
                        .map_err(|e| format!("task: {e}"))?
                    },
                    Msg::InstallDone,
                );
            }
            Msg::DlDone(Err(e)) => {
                self.animating = false;
                self.page = Page::Error(e);
            }
            Msg::InstallDone(Ok(())) => {
                self.target_pct = 100.0;
                self.animating = true;
                self.done_pending = Some((
                    String::from("Installation Complete"),
                    format!("You can find {} in your applications menu.", consts::APP_NAME),
                ));
            }
            Msg::InstallDone(Err(e)) => {
                self.animating = false;
                self.page = Page::Error(e);
            }
            Msg::Uninstall => {
                self.page = Page::Installing;
                self.status = String::from("Uninstalling...");
                self.pct = 0.0;
                self.target_pct = 50.0;
                self.animating = true;
                self.done_pending = None;
                let dest = PathBuf::from(&self.path);
                return Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || install::uninstall(&dest))
                            .await
                            .map_err(|e| format!("task: {e}"))?
                    },
                    Msg::UninstallDone,
                );
            }
            Msg::UninstallDone(Ok(())) => {
                self.target_pct = 100.0;
                self.animating = true;
                self.done_pending = Some((
                    String::from("Uninstall Complete"),
                    format!("{} has been removed.", consts::APP_NAME),
                ));
            }
            Msg::UninstallDone(Err(e)) => {
                self.animating = false;
                self.page = Page::Error(e);
            }
            Msg::Tick => {
                let diff = self.target_pct - self.pct;
                if diff.abs() < 0.5 {
                    self.pct = self.target_pct;
                    self.animating = false;
                    if let Some((title, subtitle)) = self.done_pending.take() {
                        self.page = Page::Done { title, subtitle };
                    }
                } else {
                    self.pct += diff * 0.12;
                }
            }
            Msg::Close => {
                std::process::exit(0);
            }
        }
        Task::none()
    }
}
