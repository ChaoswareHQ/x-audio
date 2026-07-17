pub mod stream;

use crate::buffer::AudioBuffer;
use crate::output::{AudioOut, Error};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

/// Desktop audio output via cpal.
pub struct DesktopAudio {
    buffer: Arc<Mutex<AudioBuffer>>,
    sample_rate: u32,
    channels: u16,
}

impl DesktopAudio {
    /// Create a new desktop audio output and start the cpal stream.
    ///
    /// Returns `Err(Error::NoDevice)` if no audio output is available.
    pub fn new(sample_rate: u32) -> Result<Self, Error> {
        let buffer = Arc::new(Mutex::new(AudioBuffer::new()));
        let buf = Arc::clone(&buffer);

        // Pre-fill ~2 frames of silence
        let frames_to_fill = (sample_rate as f64 / 60.0 * 2.0) as usize;
        buffer.lock().unwrap().fill_silence(frames_to_fill);

        let host = cpal::default_host();
        let device = host.default_output_device().ok_or(Error::NoDevice)?;
        let supported = device.default_output_config().map_err(|_| Error::NoDevice)?;
        let channels = supported.channels();
        let config: cpal::StreamConfig = supported.into();

        let stream = device
            .build_output_stream(
                config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut buf = buf.lock().unwrap();
                    for frame in data.chunks_mut(channels as usize) {
                        let s = buf.pop().unwrap_or(0.0);
                        for sample in frame.iter_mut() {
                            *sample = s;
                        }
                    }
                },
                |e| eprintln!("audio error: {e}"),
                None,
            )
            .map_err(|_| Error::Stream)?;

        stream.play().map_err(|_| Error::Stream)?;

        Ok(Self {
            buffer,
            sample_rate,
            channels,
        })
    }
}

impl AudioOut for DesktopAudio {
    fn write(&mut self, samples: &[f32]) -> usize {
        self.buffer.lock().unwrap().push(samples)
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        self.channels
    }
}
