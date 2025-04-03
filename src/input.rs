use anyhow::{Context, Result};
use log::{debug, info};
use nix::unistd::Uid;
use std::process::Command;

pub struct InputSimulator {
    simulate: bool,
}

impl InputSimulator {
    pub fn new(simulate: bool) -> Self {
        Self { simulate }
    }
    
    pub fn type_text(&self, text: &str) -> Result<()> {
        if self.simulate {
            info!("Simulating input: {}", text);
            return Ok(());
        }
        
        // Check if we have root privileges
        if !Uid::effective().is_root() {
            return Err(anyhow::anyhow!(
                "Root privileges required for input simulation. Run with sudo or use --simulate flag"
            ));
        }
        
        // Use ydotool for input simulation
        let output = Command::new("ydotool")
            .arg("type")
            .arg(text)
            .output()
            .context("Failed to execute ydotool")?;
            
        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "ydotool failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        debug!("Successfully typed text: {}", text);
        Ok(())
    }
} 