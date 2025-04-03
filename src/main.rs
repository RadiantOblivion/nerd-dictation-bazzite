mod audio;
mod capture;
mod input;
mod utils;
mod pipx;

use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info};
use std::path::PathBuf;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use thiserror::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;

use crate::audio::AudioProcessor;
use crate::capture::AudioCapture;
use crate::input::InputSimulator;
use crate::utils::{file_age_in_seconds, file_remove_if_exists, TEMP_COOKIE_NAME};
use crate::pipx::PipxWrapper;

#[derive(Error, Debug)]
pub enum NerdDictationError {
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    #[error("File operation failed: {0}")]
    FileOperation(String),
    #[error("VOSK API error: {0}")]
    VoskError(String),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the VOSK model directory
    #[arg(short, long)]
    model: Option<PathBuf>,
    
    /// Simulate input instead of actually typing
    #[arg(short, long)]
    simulate: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Use pipx environment
    #[arg(short, long)]
    pipx: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    
    let args = Args::parse();
    
    if args.pipx {
        let wrapper = PipxWrapper::new();
        wrapper.activate_vosk()?;
    }
    
    if let Err(e) = run(args) {
        error!("Error: {}", e);
        process::exit(1);
    }
    
    Ok(())
}

fn run(args: Args) -> Result<()> {
    info!("Starting nerd-dictation...");
    
    // Initialize audio processor
    let model_path = args.model.unwrap_or_else(|| {
        PathBuf::from("/usr/share/vosk-models/en-us")
    });
    
    let mut audio_processor = AudioProcessor::new(&model_path)
        .context("Failed to initialize audio processor")?;
    
    // Initialize input simulator
    let input_simulator = InputSimulator::new(args.simulate);
    
    // Initialize audio capture
    let audio_capture = AudioCapture::new(16000 * 2)?; // 2 seconds buffer
    audio_capture.start()?;
    
    // Create temp cookie file
    let temp_dir = std::env::temp_dir();
    let cookie_file = temp_dir.join(TEMP_COOKIE_NAME);
    utils::touch(&cookie_file, None)?;
    
    // Main processing loop
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;
    
    while running.load(Ordering::SeqCst) {
        // Check cookie file age
        if let Ok(age) = file_age_in_seconds(&cookie_file) {
            if age > 300.0 { // 5 minutes
                info!("Cookie file too old, shutting down...");
                break;
            }
        }
        
        // Read audio samples
        let samples = audio_capture.read_samples(16000); // 1 second of audio
        
        if samples.is_empty() {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        
        // Process audio
        if let Some(text) = audio_processor.process_audio(&samples)? {
            // Type the recognized text
            input_simulator.type_text(&text)?;
        }
    }
    
    // Finalize any remaining audio
    if let Some(text) = audio_processor.finalize()? {
        input_simulator.type_text(&text)?;
    }
    
    audio_capture.stop();
    file_remove_if_exists(&cookie_file);
    info!("Shutting down...");
    Ok(())
} 