#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod win;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use crate::consts;

pub fn default_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        dirs::data_local_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Local")))
            .expect("could not find user data dir")
            .join(consts::APP_NAME)
    }
    #[cfg(target_os = "linux")]
    {
        dirs::data_local_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".local/share")))
            .expect("could not find user data dir")
            .join(consts::APP_ID)
    }
}

fn kill_running() {
    #[cfg(target_os = "windows")]
    {
        Command::new("taskkill")
            .args(["/F", "/IM", &format!("{}.exe", consts::APP_ID)])
            .output()
            .ok();
        thread::sleep(Duration::from_millis(500));
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("pkill")
            .args(["-f", consts::APP_ID])
            .output()
            .ok();
        thread::sleep(Duration::from_millis(500));
    }
}

fn force_remove(path: &Path) -> Result<(), String> {
    // Clear read-only on all files first (Windows needs this)
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    force_remove(&p).ok();
                } else {
                    if let Ok(meta) = fs::metadata(&p) {
                        let mut perms = meta.permissions();
                        #[allow(clippy::permissions_set_readonly_false)]
                        perms.set_readonly(false);
                        fs::set_permissions(&p, perms).ok();
                    }
                    fs::remove_file(&p).ok();
                }
            }
        }
    }

    // Retry removal a few times (handles brief locks from AV scanners etc)
    for i in 0..5 {
        match fs::remove_dir_all(path) {
            Ok(()) => return Ok(()),
            Err(_) if i < 4 => thread::sleep(Duration::from_millis(500)),
            Err(e) => return Err(format!("remove {}: {e}", path.display())),
        }
    }
    Ok(())
}

pub fn is_installed(dest: &Path) -> bool {
    dest.is_dir() && fs::read_dir(dest).map(|mut d| d.next().is_some()).unwrap_or(false)
}

pub fn run(bin: &Path, dest: &Path, icon: Option<&Path>, extras: &[PathBuf]) -> Result<(), String> {
    kill_running();
    if dest.exists() {
        force_remove(dest)?;
    }
    fs::create_dir_all(dest).map_err(|e| format!("mkdir: {e}"))?;

    let exe = dest.join(bin.file_name().unwrap_or_default());
    fs::copy(bin, &exe).map_err(|e| format!("copy: {e}"))?;
    fs::remove_file(bin).ok();

    if let Some(src) = icon {
        let dst = dest.join(src.file_name().unwrap_or_default());
        fs::copy(src, &dst).map_err(|e| format!("copy icon: {e}"))?;
        fs::remove_file(src).ok();
    }

    for extra in extras {
        let dst = dest.join(extra.file_name().unwrap_or_default());
        fs::copy(extra, &dst).map_err(|e| format!("copy extra: {e}"))?;
        fs::remove_file(extra).ok();
    }

    #[cfg(target_os = "linux")]
    linux::setup(&exe, dest)?;

    #[cfg(target_os = "windows")]
    win::setup(&exe, dest)?;

    Ok(())
}

pub fn uninstall(dest: &Path) -> Result<(), String> {
    kill_running();
    #[cfg(target_os = "linux")]
    linux::teardown()?;

    #[cfg(target_os = "windows")]
    win::teardown()?;

    if dest.exists() {
        force_remove(dest)?;
    }
    Ok(())
}
