/// A lock-free ring buffer for mono audio samples.
///
/// Writer pushes samples from the emulator (APU).
/// Reader pulls samples for the audio device (cpal / I2S DMA).
pub struct AudioBuffer {
    data: [f32; Self::CAPACITY],
    write: usize,
    read: usize,
    count: usize,
}

impl AudioBuffer {
    const CAPACITY: usize = 4096;

    #[must_use]
    pub const fn new() -> Self {
        Self {
            data: [0.0f32; Self::CAPACITY],
            write: 0,
            read: 0,
            count: 0,
        }
    }

    /// Push samples from the emulator. Returns the number pushed.
    pub fn push(&mut self, samples: &[f32]) -> usize {
        let mut pushed = 0;
        for &s in samples {
            if self.count == Self::CAPACITY {
                break; // buffer full, drop
            }
            // SAFETY: write is always < CAPACITY.
            unsafe {
                *self.data.get_unchecked_mut(self.write) = s;
            }
            self.write = (self.write + 1) % Self::CAPACITY;
            self.count += 1;
            pushed += 1;
        }
        pushed
    }

    /// Pop a single sample for the audio device.
    #[must_use]
    pub fn pop(&mut self) -> Option<f32> {
        if self.count == 0 {
            return None;
        }
        // SAFETY: read is always < CAPACITY when count > 0.
        let s = unsafe { *self.data.get_unchecked(self.read) };
        self.read = (self.read + 1) % Self::CAPACITY;
        self.count -= 1;
        Some(s)
    }

    /// Number of samples available.
    #[must_use]
    pub fn available(&self) -> usize {
        self.count
    }

    /// Fill silence (pre-buffer startup).
    pub fn fill_silence(&mut self, n: usize) {
        for _ in 0..n.min(Self::CAPACITY - self.count) {
            // SAFETY: write is always < CAPACITY.
            unsafe {
                *self.data.get_unchecked_mut(self.write) = 0.0;
            }
            self.write = (self.write + 1) % Self::CAPACITY;
            self.count += 1;
        }
    }
}
