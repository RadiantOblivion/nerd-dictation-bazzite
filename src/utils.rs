use anyhow::{Context, Result};
use log::{debug, error};
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::sys::stat;
use std::fs;
use std::os::unix::prelude::*;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub const TEMP_COOKIE_NAME: &str = "nerd-dictation.cookie";
pub const USER_CONFIG_DIR: &str = "nerd-dictation";
pub const USER_CONFIG: &str = "nerd-dictation.py";

pub fn run_command_or_exit_on_failure(cmd: &[&str]) -> Result<()> {
    let output = Command::new(cmd[0])
        .args(&cmd[1..])
        .output()
        .context(format!("Failed to execute command: {:?}", cmd))?;
        
    if !output.status.success() {
        error!(
            "Command failed: {}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
        std::process::exit(1);
    }
    Ok(())
}

pub fn touch(filepath: &Path, mtime: Option<u64>) -> Result<()> {
    if filepath.exists() {
        if let Some(mtime) = mtime {
            fs::set_file_times(filepath, mtime, mtime)?;
        }
    } else {
        fs::File::create(filepath)?;
        if let Some(mtime) = mtime {
            fs::set_file_times(filepath, mtime, mtime)?;
        }
    }
    Ok(())
}

pub fn file_mtime_or_none(filepath: &Path) -> Option<u64> {
    match fs::metadata(filepath) {
        Ok(metadata) => Some(metadata.modified()?.duration_since(UNIX_EPOCH).ok()?.as_secs()),
        Err(_) => None,
    }
}

pub fn file_age_in_seconds(filepath: &Path) -> Result<f64> {
    let metadata = fs::metadata(filepath)?;
    let modified = metadata.modified()?.duration_since(UNIX_EPOCH)?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok((now - modified).as_secs_f64())
}

pub fn file_remove_if_exists(filepath: &Path) -> bool {
    match fs::remove_file(filepath) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn file_handle_make_non_blocking(file: &mut fs::File) -> Result<()> {
    let flags = fcntl(file.as_raw_fd(), FcntlArg::F_GETFL)?;
    let new_flags = OFlag::from_bits_truncate(flags) | OFlag::O_NONBLOCK;
    fcntl(file.as_raw_fd(), FcntlArg::F_SETFL(new_flags))?;
    Ok(())
} 