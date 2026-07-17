/// Error type for audio output.
#[derive(Debug)]
pub enum Error {
    /// No audio device available.
    NoDevice,
    /// Stream could not be built or started.
    Stream,
    /// Backend not supported on this platform.
    Unsupported,
}

/// Common trait for audio output.
pub trait AudioOut {
    /// Push samples for playback. Returns number actually queued.
    fn write(&mut self, samples: &[f32]) -> usize;

    /// Get the sample rate in Hz.
    fn sample_rate(&self) -> u32;

    /// Get the number of channels (1 = mono).
    fn channels(&self) -> u16;
}
