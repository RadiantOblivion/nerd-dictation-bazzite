use anyhow::{Context, Result};
use log::{debug, error, info};
use std::env;
use std::path::PathBuf;
use std::process::Command;

pub struct PipxWrapper {
    state_file: PathBuf,
    sound_begin: PathBuf,
    sound_end: PathBuf,
}

impl PipxWrapper {
    pub fn new() -> Self {
        let home = dirs::home_dir().expect("Could not find home directory");
        Self {
            state_file: home.join(".nerd-dictation-state"),
            sound_begin: home.join(".config/nerd-dictation/sounds/sound-begin.mp3"),
            sound_end: home.join(".config/nerd-dictation/sounds/sound-end.mp3"),
        }
    }
    
    pub fn activate_vosk(&self) -> Result<()> {
        let vosk_env = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".local/share/pipx/venvs/vosk/bin/activate");
            
        if !vosk_env.exists() {
            error!("Vosk environment not found at {:?}", vosk_env);
            std::process::exit(1);
        }
        
        // Source the activate script and get the environment variables
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!("source {} && env", vosk_env.display()))
            .output()
            .context("Failed to activate vosk environment")?;
            
        if !output.status.success() {
            error!(
                "Failed to activate vosk environment: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            std::process::exit(1);
        }
        
        // Update environment variables
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Some((key, value)) = line.split_once('=') {
                env::set_var(key, value);
            }
        }
        
        Ok(())
    }
    
    pub fn read_state(&self) -> Result<i32> {
        if !self.state_file.exists() {
            std::fs::write(&self.state_file, "0")?;
            return Ok(0);
        }
        
        let state = std::fs::read_to_string(&self.state_file)?;
        Ok(state.trim().parse()?)
    }
    
    pub fn write_state(&self, value: i32) -> Result<()> {
        std::fs::write(&self.state_file, value.to_string())?;
        Ok(())
    }
    
    pub fn run_nerd_dictation(&self, args: &[String]) -> Result<()> {
        let nerd_dictation_path = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".local/bin/nerd-dictation");
            
        if !nerd_dictation_path.exists() {
            error!("nerd-dictation not found at {:?}", nerd_dictation_path);
            std::process::exit(1);
        }
        
        let status = Command::new(nerd_dictation_path)
            .args(args)
            .status()
            .context("Failed to run nerd-dictation")?;
            
        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
        
        Ok(())
    }
    
    pub fn play_sound(&self, sound_file: &PathBuf) -> Result<()> {
        if sound_file.exists() {
            Command::new("mpv")
                .arg("--no-terminal")
                .arg(sound_file)
                .status()
                .context("Failed to play sound")?;
        }
        Ok(())
    }
} 