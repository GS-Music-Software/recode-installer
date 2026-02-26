use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use crate::consts;

fn bindir() -> PathBuf {
    dirs::home_dir().expect("no home dir").join(".local/bin")
}

fn desktopdir() -> PathBuf {
    dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".local/share")))
        .expect("no data dir")
        .join("applications")
}

pub fn teardown() -> Result<(), String> {
    let link = bindir().join(consts::APP_ID);
    if link.symlink_metadata().is_ok() {
        fs::remove_file(&link).ok();
    }
    let desktop = desktopdir().join(format!("{}.desktop", consts::APP_ID));
    if desktop.exists() {
        fs::remove_file(&desktop).ok();
    }
    Ok(())
}

pub fn setup(exe: &Path, dest: &Path) -> Result<(), String> {
    fs::set_permissions(exe, fs::Permissions::from_mode(0o755))
        .map_err(|e| format!("chmod: {e}"))?;
    mksymlink(exe)?;
    mkdesktop(exe, dest)?;
    Ok(())
}

fn mksymlink(exe: &Path) -> Result<(), String> {
    let bin = bindir();
    fs::create_dir_all(&bin).map_err(|e| format!("mkdir bin: {e}"))?;
    let link = bin.join(consts::APP_ID);
    if link.exists() || link.symlink_metadata().is_ok() {
        fs::remove_file(&link).ok();
    }
    std::os::unix::fs::symlink(exe, &link).map_err(|e| format!("symlink: {e}"))?;
    Ok(())
}

fn mkdesktop(exe: &Path, dest: &Path) -> Result<(), String> {
    let dir = desktopdir();
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir desktop: {e}"))?;
    let path = dir.join(format!("{}.desktop", consts::APP_ID));

    let icon = dest.join(consts::ICON_NAME);
    let icon_line = if icon.exists() {
        format!("Icon={}\n", icon.display())
    } else {
        String::new()
    };

    let content = format!(
        "[Desktop Entry]\n\
         Name={}\n\
         Exec={}\n\
         {}\
         Type=Application\n\
         Categories=Audio;Music;\n\
         Terminal=false\n",
        consts::APP_NAME,
        exe.display(),
        icon_line,
    );
    fs::write(&path, content).map_err(|e| format!("write desktop: {e}"))?;
    Ok(())
}
