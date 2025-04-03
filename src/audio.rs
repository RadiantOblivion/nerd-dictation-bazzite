use anyhow::{Context, Result};
use log::{debug, error, info};
use std::path::Path;
use vosk::{Model, Recognizer};

pub struct AudioProcessor {
    model: Model,
    recognizer: Recognizer,
}

impl AudioProcessor {
    pub fn new(model_path: &Path) -> Result<Self> {
        info!("Loading VOSK model from {:?}", model_path);
        let model = Model::new(model_path.to_str().unwrap())
            .context("Failed to load VOSK model")?;
            
        let recognizer = Recognizer::new(&model, 16000.0)
            .context("Failed to create VOSK recognizer")?;
            
        Ok(Self { model, recognizer })
    }
    
    pub fn process_audio(&mut self, audio_data: &[i16]) -> Result<Option<String>> {
        if self.recognizer.accept_waveform(audio_data) {
            let result = self.recognizer.result();
            debug!("Recognition result: {}", result);
            return Ok(Some(result));
        }
        Ok(None)
    }
    
    pub fn finalize(&mut self) -> Result<Option<String>> {
        let result = self.recognizer.final_result();
        debug!("Final recognition result: {}", result);
        Ok(Some(result))
    }
} 