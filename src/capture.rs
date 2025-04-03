use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use log::{debug, error, info};
use ringbuf::RingBuffer;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct AudioCapture {
    running: Arc<AtomicBool>,
    buffer: Arc<RingBuffer<i16>>,
}

impl AudioCapture {
    pub fn new(buffer_size: usize) -> Result<Self> {
        let running = Arc::new(AtomicBool::new(true));
        let buffer = Arc::new(RingBuffer::new(buffer_size));
        
        Ok(Self { running, buffer })
    }
    
    pub fn start(&self) -> Result<()> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .context("No input device available")?;
            
        let config = device.default_input_config()
            .context("Failed to get default input config")?;
            
        info!("Starting audio capture with config: {:?}", config);
        
        let running = self.running.clone();
        let buffer = self.buffer.clone();
        
        let stream = match config.sample_format() {
            SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if !running.load(Ordering::Relaxed) {
                        return;
                    }
                    
                    for &sample in data {
                        let sample_i16 = (sample * i16::MAX as f32) as i16;
                        if buffer.push(sample_i16).is_err() {
                            error!("Audio buffer overflow");
                        }
                    }
                },
                move |err| error!("Audio stream error: {}", err),
                None,
            )?,
            
            SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if !running.load(Ordering::Relaxed) {
                        return;
                    }
                    
                    for &sample in data {
                        if buffer.push(sample).is_err() {
                            error!("Audio buffer overflow");
                        }
                    }
                },
                move |err| error!("Audio stream error: {}", err),
                None,
            )?,
            
            SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    if !running.load(Ordering::Relaxed) {
                        return;
                    }
                    
                    for &sample in data {
                        let sample_i16 = sample as i16 - i16::MAX / 2;
                        if buffer.push(sample_i16).is_err() {
                            error!("Audio buffer overflow");
                        }
                    }
                },
                move |err| error!("Audio stream error: {}", err),
                None,
            )?,
        };
        
        stream.play()?;
        
        Ok(())
    }
    
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
    
    pub fn read_samples(&self, count: usize) -> Vec<i16> {
        let mut samples = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(sample) = self.buffer.pop() {
                samples.push(sample);
            } else {
                break;
            }
        }
        samples
    }
} 