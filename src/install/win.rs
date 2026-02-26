use std::fs;
use std::path::{Path, PathBuf};
use crate::consts;

fn startmenu() -> Option<PathBuf> {
    dirs::data_dir().map(|d| {
        d.join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
    })
}

pub fn teardown() -> Result<(), String> {
    #[cfg(windows)]
    {
        if let Some(menu) = startmenu() {
            let lnk = menu.join(format!("{}.lnk", consts::APP_NAME));
            if lnk.exists() {
                fs::remove_file(&lnk).ok();
            }
        }

        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let regpath = format!(
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            consts::APP_NAME,
        );
        hkcu.delete_subkey_all(&regpath).ok();
    }
    Ok(())
}

pub fn setup(exe: &Path, _dest: &Path) -> Result<(), String> {
    #[cfg(windows)]
    mkshortcut(exe)?;

    #[cfg(windows)]
    mkreg(exe)?;

    Ok(())
}

#[cfg(windows)]
fn mkshortcut(exe: &Path) -> Result<(), String> {
    let Some(menu) = startmenu() else {
        return Ok(());
    };
    fs::create_dir_all(&menu).ok();
    let lnk = menu.join(format!("{}.lnk", consts::APP_NAME));
    mslnk::ShellLink::new(exe)
        .map_err(|e| format!("shelllink: {e}"))?
        .create_lnk(&lnk)
        .map_err(|e| format!("create shortcut: {e}"))?;
    Ok(())
}

#[cfg(windows)]
fn mkreg(exe: &Path) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let regpath = format!(
        r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
        consts::APP_NAME,
    );
    let (key, _) = hkcu.create_subkey(&regpath).map_err(|e| format!("reg create: {e}"))?;

    key.set_value("DisplayName", &consts::APP_NAME).ok();
    key.set_value(
        "InstallLocation",
        &exe.parent().unwrap_or(exe).to_string_lossy().to_string(),
    ).ok();
    key.set_value("DisplayIcon", &exe.to_string_lossy().to_string()).ok();
    key.set_value("Publisher", &consts::PUBLISHER).ok();
    key.set_value("NoModify", &1u32).ok();
    key.set_value("NoRepair", &1u32).ok();

    Ok(())
}
