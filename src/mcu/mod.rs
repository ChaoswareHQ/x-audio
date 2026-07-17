use crate::buffer::AudioBuffer;
use crate::output::AudioOut;

/// MCU audio output via I2S (stub).
///
/// On a real MCU, samples from `write()` would be sent to an I2S DMA
/// buffer.  This stub provides the same API for testing on the host.
pub struct McuAudio {
    buffer: AudioBuffer,
    sample_rate: u32,
}

impl McuAudio {
    #[must_use]
    pub fn new(sample_rate: u32) -> Self {
        Self {
            buffer: AudioBuffer::new(),
            sample_rate,
        }
    }

    /// Read samples from the internal buffer (simulates I2S DMA read).
    #[must_use]
    pub fn read(&mut self) -> Option<f32> {
        self.buffer.pop()
    }
}

impl AudioOut for McuAudio {
    fn write(&mut self, samples: &[f32]) -> usize {
        self.buffer.push(samples)
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        1
    }
}
